<script lang="tsx" setup>
import { graphqlApi } from '@/api'
import { buildTree } from '@/utils/libs'

const categories = ref<any>([])
const fetchData = async () => {
	const { data: rData } = await graphqlApi({
		query: `
			query {
				categoryList {
					records {
						id,
						name, 
						code,
						parentId,
					}
				}
			}
		`
	})
	const categoires = rData.data.categoryList.records.map((item: { name: string; id: number; }) => ({ ...item, label: item.name, value: item.id }))
	categories.value = buildTree(categoires)
}

onMounted(() => {
	fetchData()
})
</script>

<template>
	<t-cascader :options="categories" placeholder="分类"></t-cascader>
</template>

<style lang="scss" scoped></style>