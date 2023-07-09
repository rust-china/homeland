<script lang="tsx" setup>
import LayoutHeader from './LayoutHeader.vue'
</script>

<template>
  <main class="layout">
		<header class="layout-header">
			<LayoutHeader></LayoutHeader>
		</header>
		<header class="layout-body">
			<slot></slot>
		</header>
		<header class="layout-footer main-container">footer</header>
	</main>
</template>

<style lang="scss" scoped>
</style>