<script lang="tsx" setup>
import LayoutHeader from './LayoutHeader.vue'
</script>

<template>
  <main class="layout h-full flex flex-col">
		<header class="layout-header">
			<LayoutHeader></LayoutHeader>
		</header>
		<div class="layout-body flex-1">
			<slot></slot>
		</div>
		<!-- <header class="layout-footer main-container">footer</header> -->
	</main>
</template>

<style lang="scss" scoped>
</style>