<script lang="ts" setup>
</script>

<template>
  <div class="border-2 border-cyan-600 rounded-lg">

    <!-- Heading -->
    <div class="flex">

      <!-- Title -->
      <div class="flex ml-2 mt-2">
        <div class="my-auto">
          <slot name="icon"/>
        </div>
        <div class="my-auto ml-1 font-bold text-lg ">
          <slot name="title"/>
        </div>
      </div>

      <!-- Version -->
      <slot name="heading"/>
    </div>


    <!-- Content block -->
    <div class="px-8 pb-8">
      <slot name="content"/>
    </div>

  </div>
</template>

<style scoped>

</style>
