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
    <CommandLayout version="1.0.0">
      <template v-slot:desc>
        Concatenate multiple alignments into one alignment.
      </template>

      <template v-slot:cmd>
        <b>cytos msa concat</b> [OPTIONS] &lt;OUTPUT&gt; [INPUT]...
      </template>

      <template v-slot:note>
        <FixBlock version="1.1.0">
          A bugfix has been released to address the unordered input and unsorted genome ID outputs.
        </FixBlock>
      </template>
    </CommandLayout>

    <ParamLayout class="my-3">
      <template v-slot:args>
        <div class="m-2">
          <ParamBlock name="&lt;OUTPUT&gt;" version="1.0.0">
            Write the concatenated FASTA file to this path.
          </ParamBlock>
        </div>

        <div class="m-2">
          <ParamBlock name="[INPUT]..." version="1.0.0">
            Space-separated list of FASTA files to concatenate (this will form the order).
          </ParamBlock>
        </div>
      </template>

      <template v-slot:options>
        <ParamBlock class="m-2" defaultVal="-" name="--gap" version="1.1.0">
          Missing character separator.
        </ParamBlock>
      </template>
    </ParamLayout>


    <ExampleLayout>
      <div>
        The following example takes two FASTA files <b>gene_a.fa</b> and <b>gene_b.fa</b>, then concatenates them into
        <b>output.fa</b>.
      </div>

      <div class="mt-3">
        <CodeBlock><b>cytos msa concat</b> --gap - output.fa gene_a.fa gene_b.fa</CodeBlock>
      </div>

      <!-- Files -->
      <div class="flex mt-3 flex-wrap">

        <div>
          <FileBlock name="gene_a.fa">
            >bar<br>
            CCC<br>
            >baz<br>
            TTT
          </FileBlock>
        </div>

        <div class="mx-3">
          <FileBlock name="gene_b.fa">
            >bar<br>
            TTTT<br>
            >baz<br>
            --GG<br>
            >foo<br>
            AAA-
          </FileBlock>
        </div>

        <div>

          <FileBlock name="output.fa">
            >bar<br>
            CCCTTTT<br>
            >baz<br>
            TTT--GG<br>
            >foo<br>
            ---AAA-
          </FileBlock>
        </div>
      </div>
    </ExampleLayout>

  </div>
</template>

