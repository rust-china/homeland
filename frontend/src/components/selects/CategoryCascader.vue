<script lang="tsx" setup>
import { graphqlApi } from '@/api'

const categories = ref([])
const fetchData = async () => {
	const { data: rData } = await graphqlApi({
		query: `
			query {
				categoryList {
					records {
						id,
						name, 
						code,
					}
				}
			}
		`
	})
	categories.value = rData.data.categoryList.records.map((item: { name: string; id: number; }) => ({ ...item, label: item.name, value: item.id }))
}

onMounted(() => {
	fetchData()
})
</script>

<template>
	<t-cascader :options="categories" placeholder="分类"></t-cascader>
</template>

<style lang="scss" scoped></style>