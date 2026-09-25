import { useState, useEffect, useMemo } from "react";
import Dropdown from "./components/Dropdown";
import SearchBar from "./components/SearchBar";
import styles from "./App.module.css";
import Modal from "./components/Modal";
import ToggleSwitch from "./components/ToggleSwitch";
import SearchableDropdown from "./components/SearchableDropdown";
import LineBlock from "./components/LineBlock";

const EMPTY_LINE = { items: [], error: null };
const EMPTY_LINES = { line_one: EMPTY_LINE, line_two: EMPTY_LINE, pr_6: EMPTY_LINE };

// Какие линии относятся к какому участку
const SECTION_LINES = {
  "КПЦ": ["line_one", "line_two"],
  "Пролет №6": ["pr_6"],
};

// Функция для загрузки данных с /ascan
const fetchData = async () => {
  try {
    const response = await fetch("/ascan");
    const data = await response.json();
    return data;
  } catch (error) {
    console.error("Ошибка загрузки данных:", error);
    return {
      batch_number_kpc: [], batch_number_6pr: [], malting_for_kpc: [], malting_for_6pr: [],
      errors: { server: "Сервер Асканов не отвечает" },
    };
  }
};

// Ответ сервера по одной линии -> { items: [колёса], error }
// Сервер отдаёт массивы (hot_number[i], batch_number[i], ...), собираем их в объекты колёс.
const toLine = (line) => {
  const d = line?.data;
  const items = d
    ? (d.hot_number || [])
        .map((num, i) => ({
          index: i,
          num,
          hot_number: num,
          malting_namber: d.malting_namber?.[i],
          batch_number: d.batch_number?.[i],
          task_number: d.task_number?.[i],
          path_img: d.path_img?.[i],
          date_time: d.date_time?.[i],
        }))
        .filter((w) => w.num)
    : [];
  return { items, error: line?.error ?? null };
};

const sendBatchData = async (place, batchNumber, maltingNumber, setLines) => {
  if (!batchNumber && !maltingNumber) return; // Если ничего не выбрано — не отправляем запрос

  try {
    const response = await fetch("http://192.168.131.1:8080/api/wheel", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ place, batch_number: batchNumber, malting: maltingNumber }),
    });

    if (!response.ok) throw new Error("Ошибка при отправке данных");

    // У каждой линии свой результат: если одна недоступна, остальные всё равно показываем
    const all = await response.json();
    setLines({
      line_one: toLine(all.line_one),
      line_two: toLine(all.line_two),
      pr_6: toLine(all.pr_6),
    });
  } catch (error) {
    console.error("Ошибка:", error);
    alert("Ошибка отправки данных!");
  }
};

const App = () => {
  const [selectedSection, setSelectedSection] = useState("КПЦ");
  const [selectedBatch, setSelectedBatch] = useState("");
  const [searchTerm, setSearchTerm] = useState("");
  const [batchData, setBatchData] = useState({ batch_number_kpc: [], batch_number_6pr: [] });
  const [maltingData, setMaltingData] = useState({ malting_for_kpc: [], malting_for_6pr: [] });
  const [sourceErrors, setSourceErrors] = useState({}); // линии, по которым не загрузились списки
  const [lines, setLines] = useState(EMPTY_LINES);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [wheelInfo, setWheelInfo] = useState(null);
  const [isToggled, setIsToggled] = useState(false);

  // Данные колеса уже пришли вместе со списком — отдельный запрос не нужен
  const handleWheelClick = (wheel) => {
    setWheelInfo(wheel);
    setIsModalOpen(true);
  };

  useEffect(() => {
    fetchData().then((data) => {
      setBatchData({
        batch_number_kpc: data.batch_number_kpc || [],
        batch_number_6pr: data.batch_number_6pr || [],
      });
      setMaltingData({
        malting_for_kpc: data.malting_for_kpc || [],
        malting_for_6pr: data.malting_for_6pr || [],
      });
      setSourceErrors(data.errors || {});
    });
  }, [selectedSection, isToggled]);

  useEffect(() => {
    if (!selectedBatch) return;
    if (!isToggled) {
      sendBatchData(selectedSection, selectedBatch, null, setLines);
    } else {
      sendBatchData(selectedSection, null, selectedBatch, setLines);
    }
  }, [selectedBatch, selectedSection, isToggled]);

  useEffect(() => {
    setSelectedBatch("");
    setLines(EMPTY_LINES);
  }, [isToggled, selectedSection]);

  const displayData = useMemo(() => {
    return isToggled
      ? selectedSection === 'КПЦ'
        ? maltingData.malting_for_kpc
        : maltingData.malting_for_6pr
      : selectedSection === 'КПЦ'
        ? batchData.batch_number_kpc
        : batchData.batch_number_6pr;
  }, [isToggled, selectedSection, batchData, maltingData]);

  // Ошибки загрузки списков, относящиеся к текущему участку
  const sectionErrors = Object.entries(sourceErrors)
    .filter(([key]) => key === "server" || (SECTION_LINES[selectedSection] || []).includes(key))
    .map(([, message]) => message);

  return (
    <div className={styles.container}>
      {/* Выпадающий список для выбора секции */}
      <Dropdown
        options={["КПЦ", "Пролет №6"]}
        selected={selectedSection}
        onChange={(e) => setSelectedSection(e.target.value)}
      />

      <div style={{ display: 'flex', justifyContent: 'center', margin: '20px 0' }}>
        <ToggleSwitch
          label="Включить"
          isOn={isToggled}
          handleToggle={() => {
            setIsToggled(!isToggled);
            setSelectedBatch(""); // Явный сброс при клике
          }}
        />
      </div>

      {sectionErrors.length > 0 && (
        <div className={styles.sourceWarning}>
          Список {isToggled ? "плавок" : "партий"} может быть неполным:
          <ul>
            {sectionErrors.map((msg) => <li key={msg}>{msg}</li>)}
          </ul>
        </div>
      )}

      <SearchableDropdown
        value={selectedBatch}
        options={displayData}
        placeholder={isToggled ? "Введите плавку" : "Введите партию"}
        onSelect={(value) => setSelectedBatch(value)}
      />
      {/* Поле поиска по номерам колёс */}
      <SearchBar
        placeholder={selectedSection === "КПЦ" ? "Поиск по КПЦ" : "Поиск по Пролету №6"}
        onSearch={setSearchTerm}
      />

      {isModalOpen && wheelInfo && (
        <Modal onClose={() => setIsModalOpen(false)}>
          <h2>Информация о колесе</h2>
          <p><strong>Номер колеса:</strong> {wheelInfo.num}</p>
          <div style={{ display: "flex", gap: "30px", justifyContent: "space-between", marginBottom: "20px" }}>
            <p><strong>Горячая маркировка:</strong> {wheelInfo.hot_number}</p>
            <p><strong>Плавка:</strong> {wheelInfo.malting_namber}</p>
            <p><strong>Партия:</strong> {wheelInfo.batch_number}</p>
            <p><strong>Задание:</strong> {wheelInfo.task_number}</p>
          </div>

          {wheelInfo.path_img ? (
            <img
              src={`${wheelInfo.path_img.replace("C:\\old\\rust_project\\BACK\\solo_project\\static", "").replace(/\\/g, "/")}`}
              alt="Wheel"
              style={{ width: "400%", maxWidth: "800px", marginTop: "10px", borderRadius: "8px" }}
            />
          ) : (
            <p>Картинка не найдена</p>
          )}
        </Modal>
      )}

      <div className={styles.gridContainer}>
        {selectedSection === "КПЦ" ? (
          <>
            <LineBlock
              title="Линия №1"
              line={lines.line_one}
              searchTerm={searchTerm}
              onWheelClick={handleWheelClick}
            />
            <LineBlock
              title="Линия №2"
              line={lines.line_two}
              searchTerm={searchTerm}
              onWheelClick={handleWheelClick}
            />
          </>
        ) : (
          <LineBlock
            title="Пролет №6"
            line={lines.pr_6}
            searchTerm={searchTerm}
            onWheelClick={handleWheelClick}
            showEmpty={!!selectedBatch}
          />
        )}
      </div>
    </div>
  );
};

export default App;
