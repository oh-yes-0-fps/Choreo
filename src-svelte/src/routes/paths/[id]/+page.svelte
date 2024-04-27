<script lang="ts">
    import Field from "$lib/components/field/Field.svelte"
    import Sidebar from "$lib/components/sidebar/Sidebar.svelte"
    export let data;
    $: id = data.id;
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
        <Sidebar pathId={id}></Sidebar>
        <span
            style="
            display: flex;
            flex-direction:column;
            flex-grow: 1;
            width: 0"
        >
          <!-- <Navbar></Navbar> -->
          <Field pathId={id}></Field>
        </span>
      </span>
      <!-- <PathAnimationPanel></PathAnimationPanel> -->
    </div>
  </div>
