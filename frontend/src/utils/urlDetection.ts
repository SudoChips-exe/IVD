import { Platform } from '../types'

export function detectPlatform(url: string): Platform {
  const urlLower = url.toLowerCase()

  if (urlLower.includes('instagram.com') || urlLower.includes('ig.me')) {
    return 'Instagram'
  } else if (
    urlLower.includes('tiktok.com') ||
    urlLower.includes('vm.tiktok.com') ||
    urlLower.includes('vt.tiktok.com')
  ) {
    return 'TikTok'
  } else if (urlLower.includes('youtube.com') || urlLower.includes('youtu.be')) {
    return 'YouTube'
  } else if (urlLower.includes('twitter.com') || urlLower.includes('x.com')) {
    return 'Twitter'
  } else if (urlLower.includes('facebook.com') || urlLower.includes('fb.watch')) {
    return 'Facebook'
  } else {
    return 'Unknown'
  }
}

export function isValidUrl(url: string): boolean {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}
