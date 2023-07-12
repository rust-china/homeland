import type { UnwrapNestedRefs } from 'vue'
import { trimQuery } from '@/utils/libs'

const DEFAULT_PAGE_SIZE = 20
export function useList(params: ListParams) {
	const listState: UnwrapNestedRefs<List> = reactive<List>({
		...params,
		urlQuery: params.urlQuery ?? false,
		query: params.query ?? {},
		pagination: {
			pageNo: 1, 
			pageSize: 20,
			total: 0,
			onCurrentChange(pageNo: number) {
        listState.pagination.pageNo = pageNo
        listState.onLoad()
      },
      onPageSizeChange(pageSize: number) {
        listState.pagination.pageSize = pageSize
        listState.onLoad()
      },
      ...(params.pagination ?? {})
		},
		isLoading: false,
		records: params.records ?? [],
		async onLoad(moreQuery: ListQuery = {}, shouldReset = false) {
			listState.isLoading = true
			const { pageNo, pageSize, ...customQuery } = moreQuery
			if (shouldReset) {
        listState.query = { ...params.query, ...customQuery }
        listState.pagination.pageNo = pageNo || 1
        listState.pagination.pageSize = pageSize || DEFAULT_PAGE_SIZE
      }
			if (Object.prototype.hasOwnProperty.call(moreQuery, 'pageNo')) {
        listState.pagination.pageNo = pageNo as number
      }
      if (Object.prototype.hasOwnProperty.call(moreQuery, 'pageSize')) {
        listState.pagination.pageSize = pageSize as number
      }
      if (Object.prototype.hasOwnProperty.call(listState.query, 'pageNo')) {
        listState.query.pageNo = listState.pagination.pageNo
      }
      if (Object.prototype.hasOwnProperty.call(listState.query, 'pageSize')) {
        listState.query.pageSize = listState.pagination.pageSize
      }
			try {
				await params.onLoad(trimQuery(listState.query))
			} finally {
				listState.isLoading = false
			}
		}
	})
	return listState
}

export interface ListParams extends Record<string, any> {
	urlQuery?: boolean;
	query?: ListQuery;
	pagination?: Partial<Pick<ListPagination, 'pageNo' | 'pageSize'>>;
	onLoad: (query?: any) => Promise<void>;
	records?: any[];
}

export interface ListQuery extends Record<string, any> {
  pageNo?: number
  pageSize?: number
}

export interface ListPagination {
  pageNo: number;
  pageSize: number;
  total: number;
  onCurrentChange: (pageNo: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}

export interface List extends Record<string, any> {
	urlQuery: boolean;
	query: ListQuery;
	pagination: ListPagination;
	isLoading: boolean;
	records: any[];
	onLoad: (query?: ListQuery, shouldReset?: boolean) => Promise<void>
}