import { useEffect } from 'react'

const sections = [
  { id: 'home',     path: '/' },
  { id: 'features', path: '/features' },
  { id: 'faq',      path: '/faq' },
  { id: 'contact',  path: '/contact' },
]

export function useScrollSpy() {
  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            const match = sections.find((s) => s.id === entry.target.id)
            if (match) {
              window.history.replaceState(null, '', match.path)
            }
          }
        }
      },
      { rootMargin: '0px 0px -70% 0px', threshold: 0 }
    )

    for (const { id } of sections) {
      const el = document.getElementById(id)
      if (el) observer.observe(el)
    }

    return () => observer.disconnect()
  }, [])
}
