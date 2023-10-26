<script lang="ts" setup>
import CodeBlock from "~/components/block/CodeBlock.vue";
import FileBlock from "~/components/block/FileBlock.vue";
import FixBlock from "~/components/block/FixBlock.vue";
import ParamBlock from "~/components/block/ParamBlock.vue";
import ParamLayout from "~/components/layout/ParamLayout.vue";
import ExampleLayout from "~/components/layout/ExampleLayout.vue";
import CommandLayout from "~/components/layout/CommandLayout.vue";
</script>


<template>
  <div>

    <!-- Command introduction -->
    <CommandLayout version="1.2.0">
      <template v-slot:desc>
        Bootstrap a multiple sequence alignment.
      </template>

      <template v-slot:cmd>
        <b>cytos msa bootstrap</b> [OPTIONS] &lt;INPUT&gt; &lt;OUTPUT&gt;
      </template>

    </CommandLayout>

    <ParamLayout class="my-3">
      <template v-slot:args>
        <div class="m-2">
          <ParamBlock name="&lt;INPUT&gt;" version="1.2.0">
            Read the FASTA file from this path.
          </ParamBlock>
        </div>

        <div class="m-2">
          <ParamBlock name="&lt;OUTPUT&gt;" version="1.2.0">
            Write the bootstrapped FASTA files to this directory.
          </ParamBlock>
        </div>
      </template>

      <template v-slot:options>
        <ParamBlock class="m-2" defaultVal="1" name="--replicates" version="1.2.0">
          Create this many bootstrap replicates.
        </ParamBlock>

        <ParamBlock class="m-2" defaultVal="" name="--seed" version="1.2.0">
          Random number generation seed (e.g. 42)
        </ParamBlock>
      </template>
    </ParamLayout>


    <ExampleLayout>
      <div>
        The following example one FASTA file <b>input.fa</b>, then sub-samples each alignment (with replacement) to the
        directory <b>/tmp/replicates</b>.
      </div>

      <div class="mt-3">
        <CodeBlock><b>cytos msa bootstrap</b> --replicates 1 input.fa /tmp/replicates</CodeBlock>
      </div>

      <!-- Files -->
      <div class="flex mt-3 flex-wrap">

        <div>
          <FileBlock name="input.fa">
            >bar<br>
            AAT<br>
            >baz<br>
            AGG
          </FileBlock>
        </div>

        <div class="ml-3">

          <FileBlock name="/tmp/replicates/input_0.fa">
            >bar<br>
            ATA<br>
            >baz<br>
            GGG
          </FileBlock>
        </div>
      </div>
    </ExampleLayout>

  </div>
</template>

