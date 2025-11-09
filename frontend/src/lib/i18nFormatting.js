/**
 * Advanced i18n utilities with pluralization and formatting
 */

/**
 * Pluralize a string based on count
 * @param {string} lang - Language code (de, en, fr, es, it)
 * @param {string} key - Translation key
 * @param {number} count - Item count
 * @returns {string} Pluralized translation
 */
export function pluralize(lang, key, count) {
  const pluralRules = {
    de: {
      // German: singular vs. plural
      zero: count === 0 ? `${key}Zero` : null,
      one: count === 1 ? `${key}` : null,
      other: count > 1 ? `${key}Plural` : null,
    },
    en: {
      // English: singular vs. plural
      one: count === 1 ? `${key}` : null,
      other: count !== 1 ? `${key}Plural` : null,
    },
    fr: {
      // French: 0 or 1 is singular, otherwise plural
      one: count <= 1 ? `${key}` : null,
      other: count > 1 ? `${key}Plural` : null,
    },
    es: {
      // Spanish: same as English/French
      one: count === 1 ? `${key}` : null,
      other: count !== 1 ? `${key}Plural` : null,
    },
    it: {
      // Italian: singular vs. plural
      one: count === 1 ? `${key}` : null,
      other: count !== 1 ? `${key}Plural` : null,
    },
  };

  const rules = pluralRules[lang] || pluralRules.en;
  for (const rule of Object.values(rules)) {
    if (rule) return rule;
  }

  return `${key}Plural`;
}

/**
 * Format date based on locale
 * @param {Date|string} date - Date to format
 * @param {string} lang - Language code
 * @param {string} format - 'short', 'long', 'full'
 * @returns {string} Formatted date
 */
export function formatDate(date, lang = 'en', format = 'short') {
  const d = new Date(date);

  const options = {
    short: { year: '2-digit', month: '2-digit', day: '2-digit' },
    long: { year: 'numeric', month: 'long', day: 'numeric' },
    full: {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      weekday: 'long',
      hour: '2-digit',
      minute: '2-digit',
    },
  };

  const localeMap = {
    de: 'de-DE',
    en: 'en-US',
    fr: 'fr-FR',
    es: 'es-ES',
    it: 'it-IT',
  };

  const locale = localeMap[lang] || 'en-US';
  return d.toLocaleDateString(locale, options[format] || options.short);
}

/**
 * Format time based on locale
 * @param {Date|string} date - Date to format
 * @param {string} lang - Language code
 * @param {boolean} includeSeconds - Include seconds
 * @returns {string} Formatted time
 */
export function formatTime(date, lang = 'en', includeSeconds = false) {
  const d = new Date(date);

  const options = {
    hour: '2-digit',
    minute: '2-digit',
    ...(includeSeconds && { second: '2-digit' }),
  };

  const localeMap = {
    de: 'de-DE',
    en: 'en-US',
    fr: 'fr-FR',
    es: 'es-ES',
    it: 'it-IT',
  };

  const locale = localeMap[lang] || 'en-US';
  return d.toLocaleTimeString(locale, options);
}

/**
 * Format datetime based on locale
 * @param {Date|string} date - Date to format
 * @param {string} lang - Language code
 * @param {string} format - 'short', 'long', 'full'
 * @returns {string} Formatted datetime
 */
export function formatDateTime(date, lang = 'en', format = 'short') {
  const d = new Date(date);

  const options = {
    short: {
      year: '2-digit',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    },
    long: {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    },
    full: {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      weekday: 'long',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    },
  };

  const localeMap = {
    de: 'de-DE',
    en: 'en-US',
    fr: 'fr-FR',
    es: 'es-ES',
    it: 'it-IT',
  };

  const locale = localeMap[lang] || 'en-US';
  return d.toLocaleString(locale, options[format] || options.short);
}

/**
 * Format number based on locale
 * @param {number} num - Number to format
 * @param {string} lang - Language code
 * @param {Object} options - Intl.NumberFormat options
 * @returns {string} Formatted number
 */
export function formatNumber(num, lang = 'en', options = {}) {
  const localeMap = {
    de: 'de-DE',
    en: 'en-US',
    fr: 'fr-FR',
    es: 'es-ES',
    it: 'it-IT',
  };

  const locale = localeMap[lang] || 'en-US';
  return new Intl.NumberFormat(locale, options).format(num);
}

/**
 * Format currency based on locale
 * @param {number} amount - Amount to format
 * @param {string} currency - Currency code (e.g., 'USD', 'EUR')
 * @param {string} lang - Language code
 * @returns {string} Formatted currency
 */
export function formatCurrency(amount, currency = 'USD', lang = 'en') {
  const localeMap = {
    de: 'de-DE',
    en: 'en-US',
    fr: 'fr-FR',
    es: 'es-ES',
    it: 'it-IT',
  };

  const locale = localeMap[lang] || 'en-US';
  return new Intl.NumberFormat(locale, {
    style: 'currency',
    currency: currency,
  }).format(amount);
}

/**
 * Format file size with locale-specific separators
 * @param {number} bytes - Size in bytes
 * @param {string} lang - Language code
 * @returns {string} Formatted file size
 */
export function formatFileSize(bytes, lang = 'en') {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const value = bytes / Math.pow(k, i);

  return formatNumber(value, lang, {
    maximumFractionDigits: 2,
    minimumFractionDigits: 0,
  }).concat(` ${sizes[i]}`);
}

/**
 * Format percentage based on locale
 * @param {number} num - Number (0-1 or 0-100)
 * @param {string} lang - Language code
 * @param {boolean} isDecimal - Is input in decimal format (0-1)?
 * @returns {string} Formatted percentage
 */
export function formatPercent(num, lang = 'en', isDecimal = true) {
  const value = isDecimal ? num : num / 100;
  const localeMap = {
    de: 'de-DE',
    en: 'en-US',
    fr: 'fr-FR',
    es: 'es-ES',
    it: 'it-IT',
  };

  const locale = localeMap[lang] || 'en-US';
  return new Intl.NumberFormat(locale, {
    style: 'percent',
    maximumFractionDigits: 2,
  }).format(value);
}

/**
 * Format relative time (e.g., "2 days ago")
 * @param {Date|string} date - Date to format
 * @param {string} lang - Language code
 * @returns {string} Formatted relative time
 */
export function formatRelativeTime(date, lang = 'en') {
  const now = new Date();
  const then = new Date(date);
  const seconds = Math.floor((now - then) / 1000);

  let unit = 'second';
  let value = seconds;

  if (seconds >= 60) {
    unit = 'minute';
    value = Math.floor(seconds / 60);
  }
  if (seconds >= 3600) {
    unit = 'hour';
    value = Math.floor(seconds / 3600);
  }
  if (seconds >= 86400) {
    unit = 'day';
    value = Math.floor(seconds / 86400);
  }
  if (seconds >= 604800) {
    unit = 'week';
    value = Math.floor(seconds / 604800);
  }
  if (seconds >= 2592000) {
    unit = 'month';
    value = Math.floor(seconds / 2592000);
  }
  if (seconds >= 31536000) {
    unit = 'year';
    value = Math.floor(seconds / 31536000);
  }

  const localeMap = {
    de: 'de-DE',
    en: 'en-US',
    fr: 'fr-FR',
    es: 'es-ES',
    it: 'it-IT',
  };

  const locale = localeMap[lang] || 'en-US';
  const rtf = new Intl.RelativeTimeFormat(locale, { numeric: 'auto' });
  return rtf.format(-value, unit);
}

/**
 * Collate/Sort strings based on locale
 * @param {Array<string>} strings - Strings to sort
 * @param {string} lang - Language code
 * @returns {Array<string>} Sorted strings
 */
export function collate(strings, lang = 'en') {
  const localeMap = {
    de: 'de-DE',
    en: 'en-US',
    fr: 'fr-FR',
    es: 'es-ES',
    it: 'it-IT',
  };

  const locale = localeMap[lang] || 'en-US';
  const collator = new Intl.Collator(locale);
  return [...strings].sort((a, b) => collator.compare(a, b));
}
