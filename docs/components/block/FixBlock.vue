<script lang="ts" setup>
const props = defineProps({
  version: {
    type: String,
    required: true
  }
})

import { mdiAlertCircle } from '@mdi/js';

</script>

<template>
 <div class="border-red-700 border-l-4 bg-gray-200 text-gray-900 text-sm rounded-md pr-2 w-fit">
   <div class="flex flex-row">
     <div class="my-auto">
       <Icon :path="mdiAlertCircle" class="w-5 h-4 text-red-700"></Icon>
     </div>
     <div class="pl-1">
       v{{ version }} - <slot></slot>
     </div>
   </div>

 </div>
</template>

<style scoped>

</style>
