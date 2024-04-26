<script lang="ts">
    import Field from "$lib/components/field/Field.svelte"
    import Sidebar from "$lib/components/sidebar/Sidebar.svelte"
</script>

<div class="App">
    <div class="Page">
      <!-- <AppMenu></AppMenu> -->
      <span
        style="
          display: flex;
          flex-direction: row;
          flex-grow: 1;
          height: 0;
          width: 100%;"
      >
        <Sidebar></Sidebar>
        <span
            style="
            display: flex;
            flex-direction:column;
            flex-grow: 1;
            width: 0"
        >
          <!-- <Navbar></Navbar> -->
          <Field></Field>
        </span>
      </span>
      <!-- <PathAnimationPanel></PathAnimationPanel> -->
    </div>
  </div>
