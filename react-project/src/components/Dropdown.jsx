import styles from "./Dropdown.module.css";

const Dropdown = ({ options, selected, onChange }) => {
  return (
    <select className={styles.dropdown} value={selected} onChange={onChange}>
      {options.map((option) => (
        <option key={option} value={option}>
          {option}
        </option>
      ))}
    </select>
  );
};

const Dropdown_batch = ({ options, selected, onChange}) => {
  return (
    <select className={styles.dropdown} value={selected} onChange={onChange}>
      {options.map((option) => (
        <option key={option} value={option}>
          {option}
        </option>
      ))}
    </select>
  );
};

export default Dropdown;
