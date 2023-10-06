<script lang="ts" setup>
const props = defineProps({
  path: {
    type: String,
    required: true
  },
})
</script>

<template>
  <svg fill="currentColor"  viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
    <path :d="path"/>
  </svg>
</template>

<style scoped>

</style>
