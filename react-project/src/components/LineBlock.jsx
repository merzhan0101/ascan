import GridButton from "./GridButton";
import styles from "./LineBlock.module.css";

// Одна линия: заголовок, статус связи и кнопки колёс.
// Если у линии ошибка, показываем её здесь же — остальные линии работают как обычно.
const LineBlock = ({ title, line, searchTerm, onWheelClick, showEmpty }) => {
  const items = searchTerm
    ? line.items.filter((w) => w.num.toString().includes(searchTerm))
    : line.items;
  const noConnection = line.error && line.items.length === 0;

  return (
    <div className={styles.line}>
      <h3>{title}</h3>

      {line.error && (
        <div className={noConnection ? styles.error : styles.warning}>
          {noConnection ? "Нет связи с линией" : "Внимание"}
          <div className={styles.details}>{line.error}</div>
        </div>
      )}

      <div className={styles.grid}>
        {items.map((wheel) => (
          <GridButton
            key={wheel.index}
            number={wheel.num}
            onClick={() => onWheelClick(wheel)}
          />
        ))}
      </div>

      {showEmpty && !noConnection && items.length === 0 && (
        <p className={styles.empty}>Ничего не найдено</p>
      )}
    </div>
  );
};

export default LineBlock;
