<script setup>
import { toRefs } from "vue";

const emit = defineEmits(["search", "update:modelValue"]);
const props = defineProps({
  modelValue: String,
  placeholder: String,
  disabled: Boolean,
});

const { modelValue, placeholder, disabled } = toRefs(props);
</script>

<template>
  <div class="relative border-b border-tertiary">
    <input
      type="text"
      class="text-gray-300 w-full pl-16 pr-32 py-4 bg-secondary"
      :placeholder="placeholder"
      :value="modelValue"
      @input="emit('update:modelValue', $event.target.value)"
      @keydown.enter="emit('search')"
    />
    <div
      class="absolute left-0 inset-y-0 flex items-center justify-center pl-8 pr-2"
      :class="modelValue ? 'text-gray-300' : 'text-gray-600'"
    >
      <slot name="icon"></slot>
    </div>
    <div class="absolute right-0 inset-y-0 flex items-center pr-8">
      <button
        class="rounded-full px-4 py-1 font-semibold"
        :class="
          !disabled
            ? 'text-gray-300 bg-gray-700 hover:bg-gray-500 hover:text-white'
            : 'text-gray-400 bg-gray-600 cursor-not-allowed'
        "
        :disabled="disabled"
        @click="emit('search')"
      >
        Search
      </button>
    </div>
  </div>
</template>
