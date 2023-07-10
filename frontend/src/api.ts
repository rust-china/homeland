import type { AxiosRequestConfig, AxiosResponse } from 'axios'
import apiAxios from './utils/apiAxios'

interface GraphqlData {
	query: String,
	variables?: Record<string, any>
}
export const graphqlApi = (data: GraphqlData, config?: AxiosRequestConfig<any>): Promise<AxiosResponse<any, any>> => apiAxios.post('/graphql', data, config)
export const logoutApi = () => apiAxios.post('/auth/logout')