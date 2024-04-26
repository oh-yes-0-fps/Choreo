<script lang="ts">
  import styles from "./InputList.module.css"
  /** The text to show before the number */
  export let title: string;
  /** The text to show before the number */
  export let suffix: string;
  /** Whether the input should be editable, or else italic and grayed out */
  export let enabled: boolean;
  /** The value of the input */
  export let number: number;
  /** The number of decimal places to show when not editing. */
  export let roundingPrecision = 3;
  export let setNumber: (newNumber: number) => void;
  export let setEnabled: (value: boolean) => void;
  /** Show a checkbox after the suffix that controls the enabled state of the input */
  export let showCheckbox = false;
  /** Whether or not to show the number when the input is disabled */
  export let showNumberWhenDisabled = true;
  /** The tooltip for the title */
  export let titleTooltip = undefined;
  /** Maximum width of the number input, in monospace characters */
  export let maxWidthCharacters=10;

  let focused = false;
  let editing = false;
  let editedValue = "";
  let inputElemRef: HTMLInputElement;



  const handleSetEnabled = (event: React.ChangeEvent<HTMLInputElement>) => {
    setEnabled(event.target.checked);
  };

  function unfocusedMode() {
      focused = false;
      editing= false;
      editedValue= number.toString();
  }

  function focusedMode() {
      focused =true,
      editing =false,
      editedValue =number.toString()
    inputElemRef.value = number.toString();
    inputElemRef.select();
  }

  function editingMode() {
      focused = true,
      editing = true
  }

  function getDisplayStr(): string {
    if (editing) {
      return editedValue;
    } else {
      if (focused) {
        return number.toString();
      } else {
        return getRoundedStr();
      }
    }
  }

  function getRoundedStr(): string {
    const precision = roundingPrecision ?? 3;
    return (
      Math.round(number * 10 ** precision) /
      10 ** precision
    ).toFixed(precision);
  }

  // componentDidUpdate(
  //   prevProps: Readonly<Props>,
  //   prevState: Readonly<State>,
  //   snapshot?: any
  // ): void {
  //   if (prevProps.number !== number) {
  //     // if the value has changed from the outside, make sure it is no longer
  //     // focused so concise precision is shown.
  //     unfocusedMode();
  //   }
  // }
    let characters = getRoundedStr().length + 3;
    if (maxWidthCharacters !== undefined) {
      characters = Math.min(characters, maxWidthCharacters);
    }
</script>
<div class="tooltip" data-tip={titleTooltip??""}>
          <span
            class={
              styles.Title +
              " " +
              (enabled ? "" : styles.Disabled) +
              " " +
              (titleTooltip === undefined ? "" : styles.Tooltip)
            }
          >
            {title}
          </span>
        </div>
        <input
        bind:this={inputElemRef}
          type="text"
          class={
            styles.Number +
            (showNumberWhenDisabled ? " " + styles.ShowWhenDisabled : "")
          }
          style={`min-width: ${characters}ch` }
          disabled={!enabled}
          
          on:click={(e) => e.stopPropagation()}
          on:focus={(e) => {
            focusedMode();
          }}
          on:blur={(e) => {
            const newNumber = parseFloat(state.editedValue);
            if (!Number.isNaN(newNumber)) {
              setNumber(newNumber);
            }
            unfocusedMode();
          }}
          on:change={(e) => {
            if (!state.editing) {
              editingMode();
            }
            setState({
              editedValue: e.target.value
            });
            e.preventDefault();
          }}
          on:keydown={(e) => {
            if (e.key == "Enter") {
              inputElemRef.blur();
              // let newNumber = parseFloat(state.editedValue);
              // if (!Number.isNaN(newNumber)) {
              //   setNumber(newNumber);
              // }
              // unfocusedMode();
            }
          }}
          value={getDisplayStr()}
          on:mousedown={(e) => {
            if (!focused) {
              focusedMode();
              e.preventDefault();
            }
          }}
          autoComplete="off"
          autoCorrect="off"
          autoCapitalize="off"
        />
        <span
          class={
            styles.Suffix + " " + (enabled ? "" : styles.Disabled)
          }
        >
          {suffix}
        </span>
        {#if showCheckbox}
          <input
            type="checkbox"
            class={styles.Checkbox}
            checked={enabled}
            onChange={handleSetEnabled}
          />
          {:else}
          <span></span>
       {/if}
