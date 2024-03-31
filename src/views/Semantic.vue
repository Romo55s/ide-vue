<template>
  <div class="bg-neutral-950 min-h-full flex justify-center items-center text-white">
    <div class="max-w-3xl p-8">
      <h1 class="text-4xl font-bold mb-4">Semantic</h1>
      <div v-if="semanticInfo.length > 0">
        <ul class="list-disc pl-6">
          <li v-for="(info, index) in semanticInfo" :key="index" class="mb-2">{{ info }}</li>
        </ul>
      </div>
      <div v-else>
        <p>No semantic information available.</p>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  data() {
    return {
      semanticInfo: [] as string[] // Array para almacenar la información semántica simulada
    };
  },
  methods: {
    // Método para simular la generación de información semántica
    generateSemanticInfo(): void {
      // Simulación de generación aleatoria de información semántica
      const randomNumber = Math.floor(Math.random() * 5); // Genera un número aleatorio entre 0 y 4
      const semanticMessages = [
        "Variable 'x' declared as integer.",
        "Function 'calculate' defined with parameters (int a, int b) and return type int.",
        "Variable 'result' initialized with value 10.",
        "Function 'calculate' called with arguments (5, 3).",
        "Variable 'result' updated with the result of the function 'calculate'."
      ];

      // Limpia la información semántica anterior y agrega nueva información
      this.semanticInfo = [];
      for (let i = 0; i < randomNumber; i++) {
        const randomIndex = Math.floor(Math.random() * semanticMessages.length);
        this.semanticInfo.push(semanticMessages[randomIndex]);
      }
    },
    // Método para redirigir a otra vista (aquí se puede implementar la redirección real)
    redirectToAnotherView(): void {
      // Simulación de redirección
      console.log("Redirigiendo a otra vista...");
    },
    // Método para manejar el click del botón
    handleClick(): void {
      this.generateSemanticInfo(); // Genera nueva información semántica
      this.redirectToAnotherView(); // Redirige a otra vista (simulado)
    }
  },
  mounted() {
    this.generateSemanticInfo(); // Genera información semántica al montar el componente (simulado)
  }
});
</script>

<style scoped>
/* Agrega cualquier estilo adicional utilizando clases de Tailwind CSS */
</style>
