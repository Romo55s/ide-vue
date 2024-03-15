<template>
    <div class="bg-neutral-900 min-h-full flex justify-center items-center text-white">
      <div class="max-w-3xl p-8">
        <h1 class="text-4xl font-bold mb-4">Syntactic</h1>
        <div v-if="errors.length > 0">
          <ul class="list-disc pl-6">
            <li v-for="(error, index) in errors" :key="index" class="mb-2">{{ error }}</li>
          </ul>
        </div>
        <div v-else>
          <p>No Syntactic errors yet</p>
        </div>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent } from 'vue';
  
  export default defineComponent({
    data() {
      return {
        errors: [] as string[] // Array para almacenar los errores simulados
      };
    },
    methods: {
      // Método para simular la generación de errores
      generateErrors(): void {
        // Simulación de generación aleatoria de errores
        const randomNumber = Math.floor(Math.random() * 5); // Genera un número aleatorio entre 0 y 4
        const errorMessages = [
          "SyntaxError: Unexpected token < in JSON at position 0 | ln: 895; col: 45",
          "ReferenceError: variableName is not defined | ln: 25; col: 3",
          "TypeError: Cannot read property 'property' of undefined | ln: 125; col: 23",
          "RangeError: Invalid array length | ln: 100; col: 12",
          "Error: Something went wrong | ln: 98; col: 45"
        ];
  
        // Limpia los errores anteriores y agrega nuevos errores
        this.errors = [];
        for (let i = 0; i < randomNumber; i++) {
          const randomIndex = Math.floor(Math.random() * errorMessages.length);
          this.errors.push(errorMessages[randomIndex]);
        }
      },
      // Método para redirigir a otra vista (aquí se puede implementar la redirección real)
      redirectToAnotherView(): void {
        // Simulación de redirección
        console.log("Redirigiendo a otra vista...");
      },
      // Método para manejar el click del botón
      handleClick(): void {
        this.generateErrors(); // Genera nuevos errores
        this.redirectToAnotherView(); // Redirige a otra vista (simulado)
      }
    },
    mounted() {
      this.generateErrors(); // Genera errores al montar el componente (simulado)
    }
  });
  </script>
  
  <style scoped>
  /* Add any additional styling using Tailwind CSS classes */
  </style>
  