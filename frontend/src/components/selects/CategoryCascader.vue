<script lang="tsx" setup>
import { computed } from 'vue'
import { useQuery, gql } from '@/utils/graphql'

const { result: queryCategories } = useQuery( gql`
	query {
		categories
	}
`)
const categories = computed(() => queryCategories.value?.categories.data.map(item => ({ ...item, label: item.name, value: item.id })) ?? [])
</script>

<template>
	<t-cascader :options="categories" placeholder="分类"></t-cascader>
</template>

<style lang="scss" scoped></style>