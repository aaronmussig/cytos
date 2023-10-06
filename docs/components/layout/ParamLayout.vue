<script lang="ts" setup>
import {mdiCog} from '@mdi/js';
import BaseLayout from "~/components/layout/BaseLayout.vue";
</script>

<template>
  <BaseLayout>
    <template v-slot:icon>
      <Icon :path="mdiCog" class="w-5 h-5 text-cyan-600"/>
    </template>

    <template v-slot:title>
      Parameters
    </template>

    <template v-slot:content>
      <!-- Options -->
      <div class="mt-2">
        <div class="font-bold">Options</div>
        <slot name="options"/>
      </div>

      <!-- Positional arguments -->
      <div class="mt-2">
        <div class="font-bold">Positional arguments</div>
        <div class="flex flex-wrap">
          <slot name="args"/>
        </div>
      </div>

    </template>
  </BaseLayout>
</template>

<style scoped>

</style>
