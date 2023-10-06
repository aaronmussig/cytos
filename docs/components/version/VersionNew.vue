<script lang="ts" setup>
const props = defineProps({
  version: {
    type: String,
    required: true
  }
})

import { mdiPlusCircle } from '@mdi/js';

</script>

<template>
 <div class="border-green-700 border-l-4 bg-gray-200 text-gray-900 text-sm rounded-md pr-2 w-fit">
   <div class="flex flex-row">
     <div class="my-auto">
       <Icon :path="mdiPlusCircle" class="w-4 h-4 text-green-700"></Icon>
     </div>
     <div class="pl-1">
       New in v{{ version }}
     </div>
   </div>
 </div>
</template>

<style scoped>

</style>
