<script lang="ts" setup>
const props = defineProps({
  name: {
    type: String,
    required: true
  }
})
</script>

<template>
  <div class="bg-neutral-950 border-2 border-neutral-600  font-mono text-gray-300 text-sm rounded-lg w-fit">
    <div class="font-bold bg-neutral-600 rounded-t-md px-1">
      <i>{{ name }}</i>
    </div>
    <div class="p-2">
      <slot/>
    </div>
  </div>
</template>

<style scoped>

</style>
