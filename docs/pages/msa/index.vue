<template>
  <div>

    <!-- Welcome message -->
    <div>
      Actions to perform on multiple sequence alignments.
    </div>

    <!-- Commands -->
    <div class="mt-8">

      <div class="font-bold">
        Commands
      </div>

      <div class="text-sm mt-2">
        <i>Click on a command to read more about it.</i>
      </div>

      <nuxt-link to="/msa/concat">
        <CodeBlock class="mt-3">
          <b>cytos msa concat</b> [OPTIONS] &lt;OUTPUT&gt; [INPUT]...<br><br>
          <span class="text-xs"><i>Concatenate multiple alignments into one alignment.</i></span>
        </CodeBlock>
      </nuxt-link>

      <nuxt-link to="/msa/bootstrap">
        <CodeBlock class="mt-3">
          <b>cytos msa bootstrap</b> [OPTIONS] &lt;INPUT&gt; &lt;OUTPUT&gt;<br><br>
          <span class="text-xs"><i>Bootstrap a multiple sequence alignment.</i></span>
        </CodeBlock>
      </nuxt-link>

    </div>


  </div>
</template>

<script lang="ts" setup>
import CodeBlock from "~/components/block/CodeBlock.vue";
</script>
