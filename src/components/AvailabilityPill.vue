<script setup lang="ts">
import { ref, onMounted } from 'vue';

const props = defineProps<{
  available: boolean,
  availableText: string,
  unavailableText: string,
}>();

const animateDot = ref(false);

const triggerAnimation = () => {
  animateDot.value = true;
  setTimeout(() => {
    animateDot.value = false;
  }, 400);
};

onMounted(() => {
  if (props.available) {
    setInterval(triggerAnimation, 2000);
  }
});
</script>

<template>
  <div class="pill" :class="available ? 'available' : 'unavailable'">
    <div class="dot" :class="available ? 'available' : 'unavailable'">
      <div v-if="animateDot" class="animate"></div>
    </div>
    <span>{{ available ? availableText : unavailableText }}</span>
  </div>
</template>

<style scoped>
.pill {
  display: inline-flex;
  align-items: center;
  padding: 0.5rem 1rem;
  border-radius: 20px;
  font-weight: 425;
  user-select: none;
  position: relative;
}

.pill.available {
  background-color: #f2fff0;
  color: #009b05;
}

.pill.unavailable {
  background-color: #fff9c4;
  color: #ff9114;
}

.dot {
  display: inline-block;
  width: 0.5rem;
  height: 0.5rem;
  margin-right: 0.55rem;
  border-radius: 100%;
  position: relative;
}

.dot.available {
  background-color: #048608;
}

.dot.unavailable {
  background-color: #ff9114;
}

@keyframes expand {
  0% {
    transform: scale(1);
    opacity: 1;
  }
  100% {
    transform: scale(3.2);
    opacity: 0;
  }
}

.animate {
  animation: expand 400ms ease-out;
  position: absolute;
  width: 0.5rem;
  height: 0.5rem;
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.4);
}

@media (prefers-color-scheme: dark) {
  .pill.available {
    background-color: #002e00;
    color: #00ff0d;
  }

  .pill.unavailable {
    background-color: #4e432e;
    color: #fff59d;
  }

  .dot.available {
    background-color: #00ff0d;
  }

  .dot.unavailable {
    background-color: #fff59d;
  }

  .animate {
    background: rgba(255, 255, 255, 0.4);
  }
}
</style>