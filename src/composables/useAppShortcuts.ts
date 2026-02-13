import { useAudioStore } from '../stores/audio'
import { useTrackStore } from '../stores/track'

/**
 * Centralized application keyboard shortcuts.
 *
 * Why this exists:
 * Tauri menu accelerators are not fully reliable for shifted symbol shortcuts
 * on macOS (for example Cmd+Shift+= / Cmd+Shift+-), especially across keyboard
 * layouts and input sources. To keep shortcuts predictable, we explicitly
 * handle those keys in the webview and match on KeyboardEvent.code.
 *
 * Why `code` instead of `key`:
 * - `key` reflects produced characters and changes with layout (+, =, etc.).
 * - `code` reflects the physical key (Equal, Minus), which is layout-stable.
 */
export const useAppShortcuts = () => {
  const audio = useAudioStore()
  const track = useTrackStore()

  const isTextInputTarget = (target: EventTarget | null) => {
    const element = target as HTMLElement | null
    if (!element) return false

    const localName = element.localName?.toLowerCase()
    if (localName === 'input' || localName === 'textarea' || localName === 'select') return true
    if (element.isContentEditable) return true
    return false
  }

  const keyListener = (e: KeyboardEvent) => {
    if (isTextInputTarget(e.target)) return

    const hasCommandModifier = e.metaKey || e.ctrlKey
    if (hasCommandModifier) {
      if (e.code === 'Equal') {
        e.preventDefault()
        if (e.shiftKey) audio.zoomIn('y')
        else audio.zoomIn('x')
      }

      if (e.code === 'Minus') {
        e.preventDefault()
        if (e.shiftKey) audio.zoomOut('y')
        else audio.zoomOut('x')
      }
    }

    if (e.metaKey || e.ctrlKey || e.altKey) return

    if (e.key === ' ') {
      e.preventDefault()
      void audio.playPause()
    }

    if (e.key === 's') {
      e.preventDefault()
      void track.shuffle()
    }

    if (e.key === 'Enter') {
      e.preventDefault()
      void audio.seek(0)
    }
  }

  const registerShortcuts = () => {
    window.addEventListener('keydown', keyListener)
  }

  const unregisterShortcuts = () => {
    window.removeEventListener('keydown', keyListener)
  }

  return { registerShortcuts, unregisterShortcuts }
}
