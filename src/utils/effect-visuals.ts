export interface ChartPoint {
  x: number
  y: number
}

export const clamp = (value: number, min: number, max: number) => {
  return Math.min(max, Math.max(min, value))
}

export const dbToLinear = (db: number) => Math.pow(10, db / 20)

export const linearToDb = (linear: number, floor = -72, ceiling = 24) => {
  const db = 20 * Math.log10(Math.max(Math.abs(linear), 1e-8))
  return clamp(db, floor, ceiling)
}

export const maxChannelDb = (values: number[], floor = -72, ceiling = 12) => {
  if (!values.length) return floor
  return linearToDb(
    values.reduce((max, value) => Math.max(max, Math.abs(value)), 0),
    floor,
    ceiling,
  )
}

export const normalizeDb = (value: number, minDb: number, maxDb: number) => {
  if (!Number.isFinite(value)) return 0
  return clamp((value - minDb) / Math.max(maxDb - minDb, 1e-6), 0, 1)
}

export const buildSvgPath = (
  points: ChartPoint[],
  mapX: (value: number) => number,
  mapY: (value: number) => number,
) => {
  if (points.length === 0) return ''

  return points
    .map((point, index) => {
      const prefix = index === 0 ? 'M' : 'L'
      return `${prefix} ${mapX(point.x).toFixed(2)} ${mapY(point.y).toFixed(2)}`
    })
    .join(' ')
}

export const buildCompressorTransferPoints = (
  thresholdDb: number,
  ratio: number,
  makeupDb: number,
  count = 64,
): ChartPoint[] => {
  const safeRatio = Math.max(ratio, 1)
  const minDb = -60
  const maxDb = 12

  return Array.from({ length: count }, (_, index) => {
    const x = minDb + (index / Math.max(count - 1, 1)) * (maxDb - minDb)
    const compressed = x <= thresholdDb ? x : thresholdDb + (x - thresholdDb) / safeRatio
    return { x, y: compressed + makeupDb }
  })
}

export const buildLimiterTransferPoints = (
  thresholdDb: number,
  kneeWidthDb: number,
  count = 64,
): ChartPoint[] => {
  const minDb = -36
  const maxDb = 12
  const knee = Math.max(kneeWidthDb, 0.1)
  const kneeStart = thresholdDb - knee / 2
  const kneeEnd = thresholdDb + knee / 2

  return Array.from({ length: count }, (_, index) => {
    const x = minDb + (index / Math.max(count - 1, 1)) * (maxDb - minDb)
    let y = x

    if (x >= kneeEnd) {
      y = thresholdDb
    } else if (x > kneeStart) {
      const t = clamp((x - kneeStart) / knee, 0, 1)
      const smooth = t * t * (3 - 2 * t)
      y = x + (thresholdDb - x) * smooth
    }

    return { x, y }
  })
}

export const buildDistortionTransferPoints = (
  gainDb: number,
  thresholdDb: number,
  count = 96,
): ChartPoint[] => {
  const gain = dbToLinear(gainDb)
  const threshold = clamp(dbToLinear(thresholdDb), 0.0001, 1)

  return Array.from({ length: count }, (_, index) => {
    const x = -1 + (index / Math.max(count - 1, 1)) * 2
    const driven = x * gain
    const clipped = clamp(driven, -threshold, threshold)
    return {
      x,
      y: clamp(clipped / threshold, -1, 1),
    }
  })
}
