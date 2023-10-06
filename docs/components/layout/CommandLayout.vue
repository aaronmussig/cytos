<script lang="ts" setup>
import {mdiConsole} from '@mdi/js';
import CodeBlock from "~/components/block/CodeBlock.vue";
import BaseLayout from "~/components/layout/BaseLayout.vue";

const props = defineProps({
  version: {
    type: String,
    required: true,
  },
})

</script>

<template>
  <BaseLayout>
    <template v-slot:icon>
      <Icon :path="mdiConsole" class="w-5 h-5 text-cyan-600"/>
    </template>

    <template v-slot:title>
      Command
    </template>

    <template v-slot:heading>
      <!-- Version -->
      <div class="ml-auto rounded-bl-md rounded-tr-sm mr-0 p-1 bg-cyan-600 text-sm h-fit w-fit">
        v{{ version }}
      </div>
    </template>

    <template v-slot:content>
      <div class="mt-3">
        <slot name="desc"/>
      </div>

      <div class="mt-3">
        <CodeBlock>
          <slot name="cmd"/>
        </CodeBlock>
      </div>

      <div class="mt-4">
        <slot name="note"/>
      </div>
    </template>

  </BaseLayout>
</template>

<style scoped>

</style>
