use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tokio::time::{sleep, Duration};
use sqlx::SqlitePool;
use anyhow::Result;

/// Conversion Worker - Processes pending conversion jobs in background
/// 
/// This service polls the conversion_jobs table every 5 seconds,
/// picks up pending jobs, and processes them using external tools.
pub struct ConversionWorker {
    pool: SqlitePool,
    data_dir: PathBuf,
    tools: ToolsAvailable,
}

#[derive(Debug, Clone)]
pub struct ToolsAvailable {
    pub ffmpeg: Option<String>,
    pub libreoffice: Option<String>,
    pub imagemagick: Option<String>,
    pub image_crate: bool, // Always available (Rust crate)
}

impl ConversionWorker {
    pub fn new(pool: SqlitePool, data_dir: PathBuf) -> Self {
        let tools = Self::detect_tools();
        eprintln!("[ConversionWorker] Tool availability:");
        eprintln!("  FFmpeg: {:?}", tools.ffmpeg);
        eprintln!("  LibreOffice: {:?}", tools.libreoffice);
        eprintln!("  ImageMagick: {:?}", tools.imagemagick);
        eprintln!("  image crate: {}", tools.image_crate);

        Self {
            pool,
            data_dir,
            tools,
        }
    }

    /// Detect available conversion tools on system
    fn detect_tools() -> ToolsAvailable {
        ToolsAvailable {
            ffmpeg: Self::check_command("ffmpeg", &["-version"]),
            libreoffice: Self::check_command("soffice", &["--version"])
                .or_else(|| Self::check_command("libreoffice", &["--version"])),
            imagemagick: Self::check_command("convert", &["-version"])
                .or_else(|| Self::check_command("magick", &["-version"])),
            image_crate: true,
        }
    }

    /// Check if a command is available and return version string
    fn check_command(cmd: &str, args: &[&str]) -> Option<String> {
        Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout)
                        .ok()
                        .and_then(|s| s.lines().next().map(|l| l.to_string()))
                } else {
                    None
                }
            })
    }

    /// Start the worker loop (runs forever)
    pub async fn start(self) {
        eprintln!("[ConversionWorker] Starting worker loop...");
        
        loop {
            if let Err(e) = self.process_pending_jobs().await {
                eprintln!("[ConversionWorker] Error processing jobs: {}", e);
            }
            
            // Poll every 5 seconds
            sleep(Duration::from_secs(5)).await;
        }
    }

    /// Process all pending jobs
    async fn process_pending_jobs(&self) -> Result<(), anyhow::Error> {
        // Fetch pending jobs
        let jobs: Vec<Job> = sqlx::query_as(
            "SELECT * FROM conversion_jobs 
            WHERE status = 'pending' 
            ORDER BY created_at ASC 
            LIMIT 5"
        )
        .fetch_all(&self.pool)
        .await?;

        for job in jobs {
            eprintln!("[ConversionWorker] Processing job: {}", job.id);
            
            // Mark as processing
            sqlx::query(
                "UPDATE conversion_jobs 
                SET status = 'processing', started_at = CURRENT_TIMESTAMP 
                WHERE id = ?"
            )
            .bind(&job.id)
            .execute(&self.pool)
            .await?;

            // Process the conversion
            match self.convert_file(&job).await {
                Ok(output_path) => {
                    // Get file size
                    let file_size = tokio::fs::metadata(&output_path)
                        .await
                        .ok()
                        .map(|m| m.len() as i64);

                    // Mark as completed
                    sqlx::query(
                        "UPDATE conversion_jobs 
                        SET status = 'completed', 
                            output_file_path = ?, 
                            file_size_bytes = ?,
                            progress = 100,
                            completed_at = CURRENT_TIMESTAMP 
                        WHERE id = ?"
                    )
                    .bind(output_path.to_string_lossy().to_string())
                    .bind(file_size)
                    .bind(&job.id)
                    .execute(&self.pool)
                    .await?;

                    eprintln!("[ConversionWorker] Job {} completed successfully", job.id);
                }
                Err(e) => {
                    // Mark as failed
                    let error_msg = e.to_string();
                    sqlx::query(
                        "UPDATE conversion_jobs 
                        SET status = 'failed', 
                            error_message = ?,
                            completed_at = CURRENT_TIMESTAMP 
                        WHERE id = ?"
                    )
                    .bind(&error_msg)
                    .bind(&job.id)
                    .execute(&self.pool)
                    .await?;

                    eprintln!("[ConversionWorker] Job {} failed: {}", job.id, error_msg);
                }
            }
        }

        Ok(())
    }

    /// Convert a single file
    async fn convert_file(&self, job: &Job) -> Result<PathBuf, anyhow::Error> {
        // Create output directory: data/conversions/{user_id}/{job_id}/
        let output_dir = self.data_dir
            .join("conversions")
            .join(&job.user_id)
            .join(&job.id);
        tokio::fs::create_dir_all(&output_dir).await?;

        // Determine output filename
        let source_path = Path::new(&job.source_file_path);
        let source_filename = source_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("converted");
        let output_filename = format!("{}.{}", source_filename, job.target_format);
        let output_path = output_dir.join(output_filename);

        // Determine conversion strategy
        let category = self.get_format_category(&job.target_format);

        match category.as_str() {
            "image" => self.convert_image(source_path, &output_path, job).await?,
            "document" => self.convert_document(source_path, &output_path, job).await?,
            "video" => self.convert_video(source_path, &output_path, job).await?,
            "audio" => self.convert_audio(source_path, &output_path, job).await?,
            _ => return Err(anyhow::anyhow!("Unsupported target format: {}", job.target_format)),
        }

        Ok(output_path)
    }

    fn get_format_category(&self, format: &str) -> String {
        match format.to_lowercase().as_str() {
            "jpg" | "jpeg" | "png" | "webp" | "gif" | "bmp" | "heic" | "avif" => "image".to_string(),
            "pdf" | "docx" | "doc" | "odt" | "rtf" | "txt" | "md" | "html" => "document".to_string(),
            "mp4" | "webm" | "avi" | "mov" | "mkv" | "flv" | "wmv" => "video".to_string(),
            "mp3" | "wav" | "ogg" | "flac" | "m4a" | "aac" | "wma" => "audio".to_string(),
            _ => "unknown".to_string(),
        }
    }

    /// Convert image using image crate
    async fn convert_image(
        &self,
        source: &Path,
        output: &Path,
        job: &Job,
    ) -> Result<(), anyhow::Error> {
        // Parse options
        let options: ImageOptions = if let Some(opts) = &job.conversion_options {
            serde_json::from_str(opts).unwrap_or_default()
        } else {
            ImageOptions::default()
        };

        // Use image crate for conversion (blocking operation)
        let source = source.to_path_buf();
        let output = output.to_path_buf();
        
        tokio::task::spawn_blocking(move || {
            use image::{ImageFormat, ImageReader};

            // Load image
            let img = ImageReader::open(&source)?
                .decode()?;

            // Apply resize if specified
            let img = if let Some(resize) = options.resize {
                img.resize(
                    resize.width,
                    resize.height,
                    image::imageops::FilterType::Lanczos3,
                )
            } else {
                img
            };

            // Determine output format
            let format = match output.extension().and_then(|e| e.to_str()) {
                Some("jpg") | Some("jpeg") => ImageFormat::Jpeg,
                Some("png") => ImageFormat::Png,
                Some("webp") => ImageFormat::WebP,
                Some("gif") => ImageFormat::Gif,
                Some("bmp") => ImageFormat::Bmp,
                _ => ImageFormat::Png,
            };

            // Save with quality
            if format == ImageFormat::Jpeg {
                use image::codecs::jpeg::JpegEncoder;
                let file = std::fs::File::create(&output)?;
                let mut encoder = JpegEncoder::new_with_quality(file, options.quality.unwrap_or(85));
                encoder.encode_image(&img)?;
            } else {
                img.save_with_format(&output, format)?;
            }

            Ok::<(), anyhow::Error>(())
        })
        .await??;

        Ok(())
    }

    /// Convert document using LibreOffice
    async fn convert_document(
        &self,
        source: &Path,
        output: &Path,
        _job: &Job,
    ) -> Result<(), anyhow::Error> {
        if self.tools.libreoffice.is_none() {
            return Err(anyhow::anyhow!("LibreOffice not installed. Please install with: winget install TheDocumentFoundation.LibreOffice"));
        }

        let output_dir = output.parent().unwrap();
        let output_format = output.extension().and_then(|e| e.to_str()).unwrap_or("pdf");

        // LibreOffice command
        let status = Command::new("soffice")
            .args(&[
                "--headless",
                "--convert-to",
                output_format,
                "--outdir",
                output_dir.to_str().unwrap(),
                source.to_str().unwrap(),
            ])
            .status()?;

        if !status.success() {
            return Err(anyhow::anyhow!("LibreOffice conversion failed"));
        }

        Ok(())
    }

    /// Convert video using FFmpeg
    async fn convert_video(
        &self,
        source: &Path,
        output: &Path,
        job: &Job,
    ) -> Result<(), anyhow::Error> {
        if self.tools.ffmpeg.is_none() {
            return Err(anyhow::anyhow!("FFmpeg not installed. Please install with: winget install FFmpeg"));
        }

        // Parse options
        let options: VideoOptions = if let Some(opts) = &job.conversion_options {
            serde_json::from_str(opts).unwrap_or_default()
        } else {
            VideoOptions::default()
        };

        let mut args = vec![
            "-i", source.to_str().unwrap(),
            "-y", // Overwrite output
        ];

        // Video codec
        if let Some(codec) = &options.codec {
            args.extend(&["-c:v", codec]);
        }

        // Audio codec
        if let Some(audio_codec) = &options.audio_codec {
            args.extend(&["-c:a", audio_codec]);
        }

        // Bitrate
        if let Some(bitrate) = &options.bitrate {
            args.extend(&["-b:v", bitrate]);
        }

        // Output
        args.push(output.to_str().unwrap());

        let status = Command::new("ffmpeg")
            .args(&args)
            .status()?;

        if !status.success() {
            return Err(anyhow::anyhow!("FFmpeg conversion failed"));
        }

        Ok(())
    }

    /// Convert audio using FFmpeg
    async fn convert_audio(
        &self,
        source: &Path,
        output: &Path,
        job: &Job,
    ) -> Result<(), anyhow::Error> {
        if self.tools.ffmpeg.is_none() {
            return Err(anyhow::anyhow!("FFmpeg not installed. Please install with: winget install FFmpeg"));
        }

        // Parse options
        let options: AudioOptions = if let Some(opts) = &job.conversion_options {
            serde_json::from_str(opts).unwrap_or_default()
        } else {
            AudioOptions::default()
        };

        let mut args = vec![
            "-i", source.to_str().unwrap(),
            "-y",
        ];

        // Bitrate
        if let Some(bitrate) = &options.bitrate {
            args.extend(&["-b:a", bitrate]);
        }

        // Output
        args.push(output.to_str().unwrap());

        let status = Command::new("ffmpeg")
            .args(&args)
            .status()?;

        if !status.success() {
            return Err(anyhow::anyhow!("FFmpeg conversion failed"));
        }

        Ok(())
    }
}

// ============================================================================
// Types
// ============================================================================

#[derive(sqlx::FromRow, Debug)]
struct Job {
    id: String,
    user_id: String,
    source_file_path: String,
    source_format: String,
    target_format: String,
    conversion_options: Option<String>,
}

#[derive(serde::Deserialize, Default)]
struct ImageOptions {
    quality: Option<u8>,
    resize: Option<ResizeOptions>,
    strip_metadata: Option<bool>,
}

#[derive(serde::Deserialize)]
struct ResizeOptions {
    width: u32,
    height: u32,
}

#[derive(serde::Deserialize, Default)]
struct VideoOptions {
    codec: Option<String>,
    quality: Option<String>,
    resolution: Option<String>,
    bitrate: Option<String>,
    audio_codec: Option<String>,
}

#[derive(serde::Deserialize, Default)]
struct AudioOptions {
    bitrate: Option<String>,
    quality: Option<String>,
}
