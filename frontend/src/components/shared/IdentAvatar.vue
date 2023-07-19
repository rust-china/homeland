<!-- https://github.com/loweisz/generate-avatar -->
<script lang="tsx" setup>
import { generateFromString } from 'generate-avatar'

const props = defineProps({
  ident: [String, Number],
})

const url = computed(() => {
  return `data:image/svg+xml;utf8,${generateFromString(`${props.ident}`)}`
})
</script>

<template>
  <img :src="url" alt="">
</template>

<style lang="scss" scoped></style>