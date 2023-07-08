import apiAxios from './utils/apiAxios'

export const graphqlApi = (...args: any[]) => apiAxios.post('/graphql', ...args)