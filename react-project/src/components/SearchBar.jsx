import styles from "./SearchBar.module.css";

const SearchBar = ({ placeholder, onSearch }) => {
  return (
    <input
      type="text"
      className={styles.searchBar}
      placeholder={placeholder}
      onChange={(e) => onSearch(e.target.value)}
    />
  );
};

export default SearchBar;
