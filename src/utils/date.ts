import { format, formatDistanceToNow, parseISO } from 'date-fns'

/**
 * Format a date string to a readable format
 * @param dateString - ISO date string
 * @param formatString - Date format string (default: 'MMM dd, yyyy')
 * @returns Formatted date string
 */
export function formatDate(
  dateString: string,
  formatString: string = 'MMM dd, yyyy'
): string {
  try {
    const date = parseISO(dateString)
    return format(date, formatString)
  } catch (error) {
    console.error('Error formatting date:', error)
    return dateString
  }
}

/**
 * Get relative time from now (e.g., "2 days ago")
 * @param dateString - ISO date string
 * @returns Relative time string
 */
export function getRelativeTime(dateString: string): string {
  try {
    const date = parseISO(dateString)
    return formatDistanceToNow(date, { addSuffix: true })
  } catch (error) {
    console.error('Error getting relative time:', error)
    return dateString
  }
}

/**
 * Check if a date is recent (within the last 7 days)
 * @param dateString - ISO date string
 * @returns Boolean indicating if the date is recent
 */
export function isRecentDate(dateString: string): boolean {
  try {
    const date = parseISO(dateString)
    const now = new Date()
    const sevenDaysAgo = new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000)
    return date > sevenDaysAgo
  } catch (error) {
    console.error('Error checking if date is recent:', error)
    return false
  }
}

/**
 * Get the year from a date string
 * @param dateString - ISO date string
 * @returns Year as number
 */
export function getYear(dateString: string): number {
  try {
    const date = parseISO(dateString)
    return date.getFullYear()
  } catch (error) {
    console.error('Error getting year from date:', error)
    return new Date().getFullYear()
  }
}