import React, { useState, useEffect } from "react";
import styles from "./SearchableDropdown.module.css";

const SearchableDropdown = ({ 
  options = [], 
  onSelect, 
  placeholder = "Поиск...",
  value = "" // Добавляем пропс value
}) => {
  const [search, setSearch] = useState("");
  const [isOpen, setIsOpen] = useState(false);

  // Синхронизируем внутреннее состояние с внешним значением
  useEffect(() => {
    setSearch(value);
  }, [value]);

  const filteredOptions = options.filter((option) =>
    option.toLowerCase().includes(search.toLowerCase())
  );

  const handleSelect = (value) => {
    onSelect(value);
    setSearch(value);
    setIsOpen(false);
  };

  return (
    <div className={styles.dropdownContainer}>
      <input
        type="text"
        value={search}
        placeholder={placeholder}
        onChange={(e) => {
          setSearch(e.target.value);
          setIsOpen(true);
        }}
        onFocus={() => setIsOpen(true)}
        className={styles.input}
      />
      {isOpen && filteredOptions.length > 0 && (
        <ul className={styles.dropdown}>
          {filteredOptions.map((option, index) => (
            <li
              key={index}
              className={styles.option}
              onClick={() => handleSelect(option)}
            >
              {option}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default SearchableDropdown;