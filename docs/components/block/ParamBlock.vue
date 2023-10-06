<script lang="ts" setup>
import CodeBlock from "~/components/block/CodeBlock.vue";

const props = defineProps({
  name: {
    type: String,
    required: true
  },
  version: {
    type: String,
    required: true,
  },
  defaultVal: {
    type: String,
    required: false,
  }
})
</script>

<template>
  <div class="w-fit bg-gray-800 rounded-lg">

    <!-- Header -->
    <div class="flex">

      <div class="font-mono bg-black rounded-tl-md rounded-br-md font-bold text-sm w-fit p-2">
        {{ name }}
      </div>

      <div class="p-1 ml-auto text-sm h-fit bg-cyan-600 rounded-tr-md rounded-bl-md">
        v{{ version }}
      </div>

    </div>

    <!-- Body -->
    <div class="p-3">

      <div>
        <slot/>
      </div>

      <div class="mt-1" v-if="defaultVal">
      Default value: {{ defaultVal }}
      </div>
    </div>

  </div>
</template>

<style scoped>

</style>
