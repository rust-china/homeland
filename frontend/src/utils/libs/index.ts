export function trimQuery(query: Record<string, any>) {
  const newQuery: Record<string, any> = {}
  for (const key in query) {
    if ([undefined, null, ''].indexOf(query[key]) === -1) {
      newQuery[key] = query[key]
    }
  }
  return newQuery
}

export const buildTree = (list: any[], selfKey: string = 'id', parentKey: string = 'parentId'): any[] => {
  const map = {} as any
  for (const item of list) {
    Object.assign(item, { _isRoot: true })
    Object.assign(map, { [item[selfKey]]: item })
  }
  for (const item of list) {
    if (item[parentKey] && map[item[parentKey]]) {
      map[item[parentKey]].children = [...(map[item[parentKey]].children || []), item]
      item._isRoot = false
    }
  }
  return list.filter((i) => i._isRoot)
}