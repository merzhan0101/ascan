import styles from "./GridButton.module.css";

const GridButton = ({ number, onClick }) => {
  return <button className={styles.gridButton}onClick={onClick} >{number}</button>;
};

export default GridButton;
