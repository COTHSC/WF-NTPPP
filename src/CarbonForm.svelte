<script>
  import {
    Form,
    FormGroup,
    Checkbox,
    RadioButtonGroup,
    RadioButton,
    Select,
    SelectItem,
    Button,
  } from "carbon-components-svelte";
</script>

<Form on:submit>
  <FormGroup legendText="video">
    <Checkbox id="checkbox-0" labelText="Checkbox Label" checked />
  </FormGroup>
  <FormGroup legendText="Radio buttons">
    <RadioButtonGroup name="radio-button-group" selected="default-selected">
      <RadioButton
        id="radio-1"
        value="standard"
        labelText="Standard Radio Button"
      />
      <RadioButton
        id="radio-2"
        value="default-selected"
        labelText="Default Selected Radio Button"
      />
      <RadioButton
        id="radio-4"
        value="disabled"
        labelText="Disabled Radio Button"
        disabled
      />
    </RadioButtonGroup>
  </FormGroup>
  <FormGroup>
    <Select id="select-1" labelText="Select menu" value="placeholder-item">
      <SelectItem
        disabled
        hidden
        value="placeholder-item"
        text="Choose an option"
      />
      <SelectItem value="option-1" text="Option 1" />
      <SelectItem value="option-2" text="Option 2" />
      <SelectItem value="option-3" text="Option 3" />
    </Select>
  </FormGroup>
  <Button type="submit">Submit</Button>
</Form>
