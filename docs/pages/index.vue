<template>
  <div>

    <!-- Welcome message -->
    <div>
      <p>
        Welcome to the cytos documentation!
      </p>
      <p class="mt-5">
        Here you'll find a collection of essential Bioinformatics scripts in Rust. I'll continually update this with new commands as require them.
      </p>

    </div>

    <!-- Commands -->
    <div class="mt-8">

      <div class="font-bold">
        Commands
      </div>

      <div class="text-sm mt-2">
        <i>Click on a command to read more about it.</i>
      </div>

      <nuxt-link to="/msa">
        <CodeBlock class="mt-3">
          <b>cytos msa</b> [COMMAND]<br><br>
          <span class="text-xs"><i>Actions to perform on multiple sequence alignments.</i></span>
        </CodeBlock>
      </nuxt-link>


    </div>

  </div>
</template>

<script lang="ts" setup>
import CodeBlock from "~/components/block/CodeBlock.vue";
</script>
