
const App = () => {
  const setValueInSomeCustomWay = (value) => {
    setValue(value*2);
  }
  
  const [value, setValue] = useReducer(setValueInSomeCustomWay, "");
  return <div>{value}</div>;
}

