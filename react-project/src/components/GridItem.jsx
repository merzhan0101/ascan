import styles from "./GridItem.module.css";

const GridItem = ({ text }) => {
  return <div className={styles.gridItem}>{text}</div>;
};

export default GridItem;
