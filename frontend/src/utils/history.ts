import { HistoryEntry } from '../types'

const KEY = 'vidclaw_history'
const MAX = 50

export function getHistory(): HistoryEntry[] {
  try {
    return JSON.parse(localStorage.getItem(KEY) ?? '[]')
  } catch {
    return []
  }
}

export function saveToHistory(entry: Omit<HistoryEntry, 'id' | 'timestamp'>) {
  const history = getHistory()
  const newEntry: HistoryEntry = {
    ...entry,
    id: crypto.randomUUID(),
    timestamp: Date.now(),
  }
  localStorage.setItem(KEY, JSON.stringify([newEntry, ...history].slice(0, MAX)))
}

export function clearHistory() {
  localStorage.removeItem(KEY)
}
