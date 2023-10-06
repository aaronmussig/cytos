<script lang="ts" setup>
import {mdiHelpCircle} from '@mdi/js';
import BaseLayout from "~/components/layout/BaseLayout.vue";
</script>

<template>
  <BaseLayout>
    <template v-slot:icon>
      <Icon :path="mdiHelpCircle" class="w-5 h-5 text-cyan-600"/>
    </template>

    <template v-slot:title>
      Example
    </template>

    <template v-slot:content>
      <slot/>
    </template>
  </BaseLayout>

</template>

<style scoped>

</style>
