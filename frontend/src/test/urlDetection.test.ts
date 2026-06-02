import { describe, it, expect } from 'vitest'
import { detectPlatform, isValidUrl } from '../utils/urlDetection'

describe('detectPlatform', () => {
  it('detects Instagram', () => {
    expect(detectPlatform('https://www.instagram.com/reel/abc123')).toBe('Instagram')
    expect(detectPlatform('https://ig.me/p/abc')).toBe('Instagram')
  })

  it('detects TikTok', () => {
    expect(detectPlatform('https://www.tiktok.com/@user/video/123')).toBe('TikTok')
    expect(detectPlatform('https://vm.tiktok.com/abc')).toBe('TikTok')
    expect(detectPlatform('https://vt.tiktok.com/abc')).toBe('TikTok')
  })

  it('detects YouTube', () => {
    expect(detectPlatform('https://www.youtube.com/watch?v=abc')).toBe('YouTube')
    expect(detectPlatform('https://youtu.be/abc123')).toBe('YouTube')
  })

  it('detects Twitter / X', () => {
    expect(detectPlatform('https://twitter.com/user/status/123')).toBe('Twitter')
    expect(detectPlatform('https://x.com/user/status/123')).toBe('Twitter')
  })

  it('detects Facebook', () => {
    expect(detectPlatform('https://www.facebook.com/video/123')).toBe('Facebook')
    expect(detectPlatform('https://fb.watch/abc')).toBe('Facebook')
  })

  it('returns Unknown for unrecognised URLs', () => {
    expect(detectPlatform('https://example.com/video')).toBe('Unknown')
    expect(detectPlatform('not-a-url')).toBe('Unknown')
  })

  it('is case-insensitive', () => {
    expect(detectPlatform('https://INSTAGRAM.COM/reel/abc')).toBe('Instagram')
    expect(detectPlatform('HTTPS://YOUTUBE.COM/watch?v=abc')).toBe('YouTube')
  })
})

describe('isValidUrl', () => {
  it('accepts valid URLs', () => {
    expect(isValidUrl('https://www.instagram.com/reel/abc')).toBe(true)
    expect(isValidUrl('http://example.com')).toBe(true)
  })

  it('rejects invalid URLs', () => {
    expect(isValidUrl('not-a-url')).toBe(false)
    expect(isValidUrl('')).toBe(false)
    expect(isValidUrl('://no-scheme')).toBe(false)
  })
})
