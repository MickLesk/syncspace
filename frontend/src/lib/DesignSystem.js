// Lightweight design tokens for the redesign workstream.
export const spacing = {
  xs: "0.5rem",
  sm: "1rem",
  md: "1.5rem",
  lg: "2rem",
  xl: "3rem",
};

export const colors = {
  primary: "#667eea",
  secondary: "#764ba2",
  gradient: {
    start: "#667eea",
    mid: "#764ba2",
    end: "#f093fb",
  },
  glass: {
    light: "rgba(255, 255, 255, 0.75)",
    dark: "rgba(15, 23, 42, 0.75)",
    border: "rgba(255, 255, 255, 0.25)",
  },
};

export const transitions = {
  fast: "150ms",
  normal: "200ms",
  slow: "300ms",
  easing: "cubic-bezier(0.4, 0, 0.2, 1)",
};

export const effects = {
  blur: "12px",
  radius: "1rem",
  shadow: "0 20px 45px rgba(15, 23, 42, 0.2)",
};
