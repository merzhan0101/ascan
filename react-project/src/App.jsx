import { useState, useEffect, useMemo } from "react";
import Dropdown from "./components/Dropdown";
import SearchBar from "./components/SearchBar";
import GridButton from "./components/GridButton";
import styles from "./App.module.css";
import Modal from "./components/Modal";
import ToggleSwitch from "./components/ToggleSwitch";
import SearchableDropdown from "./components/SearchableDropdown";
// Функция для загрузки данных с /ascan
const fetchData = async () => {
  try {
    const response = await fetch("/ascan");
    const data = await response.json();
    return data;
  } catch (error) {
    console.error("Ошибка загрузки данных:", error);
    return { batch_number_kpc: [], batch_number_6pr: [], malting_for_kpc: [], malting_for_6pr: []};
  }
};

const sendBatchData = async (place, batchNumber, maltingNumber, setNumbers, setNumberOneLine, setNumberTwoLine) => {
  if (!batchNumber && !maltingNumber) return; // Если batchNumber не выбран — не отправляем запрос

  try {
    const response = await fetch("http://192.168.131.1:8080/api/wheel", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ place, batch_number: batchNumber, malting:maltingNumber }),
    });

    if (!response.ok) throw new Error("Ошибка при отправке данных");

    const allNumbers = await response.json(); // Получаем массив чисел от сервера
    setNumbers(allNumbers.pr_6.filter(Boolean)); // Обновляем кнопки числами
    setNumberOneLine(allNumbers.line_one.filter(Boolean));
    setNumberTwoLine(allNumbers.line_two.filter(Boolean));
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
  const [numbers, setNumbers] = useState([]); // Числа для кнопок
  const [numberOneLine, setNumberOneLine] = useState([]);
  const [numberTwoLine, setNumberTwoLine] = useState([]);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedWheel, setSelectedWheel] = useState(null);
  const [wheelInfo, setWheelInfo] = useState(null);
  const [isToggled, setIsToggled] = useState(false);

  const handleWheelClick = async (wheelNumber, place, line) => {
    try {
      const requestBody = {
        place,
        wheel_number: wheelNumber.toString(),
        line,
        [isToggled ? 'malting_number' : 'batch_number']: selectedBatch.toString()
      };
  
      const response = await fetch("http://192.168.131.1:8080/api/wheel/data", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(requestBody),
      });
  
      if (!response.ok) throw new Error("Ошибка");
  
      const result = await response.json();
      setSelectedWheel(wheelNumber);
      setWheelInfo(result);
      setIsModalOpen(true);
    } catch (error) {
      console.error("Ошибка:", error);
    }
  };

  useEffect(() => {
    fetchData().then((data) => {
      setBatchData({
        batch_number_kpc: data.batch_number_kpc,
        batch_number_6pr: data.batch_number_6pr,
      });
      setMaltingData({
        malting_for_kpc: data.malting_for_kpc,
        malting_for_6pr: data.malting_for_6pr,
      });
    });
  }, [selectedSection, isToggled]);

  useEffect(() => {
    if (!isToggled){
      if (selectedBatch) {
        sendBatchData(selectedSection, selectedBatch, null, setNumbers, setNumberOneLine, setNumberTwoLine);
        console.log("партия")
      }
    }
    else{
      if (selectedBatch) {
        sendBatchData(selectedSection, null, selectedBatch, setNumbers, setNumberOneLine, setNumberTwoLine);
        console.log("плавка")
        
      }
    }

  }, [selectedBatch, selectedSection]);

  useEffect(() => {
    setSelectedBatch(""); 
  }, [isToggled, selectedSection]);

  const displayData = useMemo(() => {
    return isToggled
      ? selectedSection === 'КПЦ' 
        ? maltingData.malting_for_kpc 
        : maltingData.malting_for_6pr
      : selectedSection === 'КПЦ' 
        ? batchData.batch_number_kpc 
        : batchData.batch_number_6pr;
  }, [isToggled, selectedSection, batchData, maltingData])
  
  const filteredNumbers = searchTerm
    ? numbers.filter((num) => num.toString().includes(searchTerm))
    : numbers;
  
  const filteredLineOne = searchTerm
    ? numberOneLine.filter((num) => num?.toString().includes(searchTerm))
    : numberOneLine;
  
  const filteredLineTwo = searchTerm
    ? numberTwoLine.filter((num) => num?.toString().includes(searchTerm))
    : numberTwoLine;
  
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
      <SearchableDropdown
        value={selectedBatch} // Добавляем пропс value
        options={displayData}
        placeholder="Введите партию"
        onSelect={(value) => setSelectedBatch(value)}
      />
      {/* Условное отображение поля поиска */}
      <SearchBar
        placeholder={selectedSection === "КПЦ" ? "Поиск по КПЦ" : "Поиск по Пролету №6"}
        onSearch={setSearchTerm}
      />

      {isModalOpen && (
        <Modal onClose={() => setIsModalOpen(false)}>
          <h2>Информация о колесе</h2>
          <p><strong>Номер колеса:</strong> {selectedWheel}</p>
          {wheelInfo && (
            <>
              <div style={{ display: "flex", gap: "30px", justifyContent: "space-between", marginBottom: "20px" }}>
                <p><strong>Горячая маркировка:</strong> {wheelInfo.hot_number}</p>
                <p><strong>Плавка:</strong> {wheelInfo.malting_namber}</p>
                <p><strong>Партия:</strong> {wheelInfo.batch_number}</p>
                <p><strong>Задание:</strong> {wheelInfo.task_number}</p>
              </div>
          
              {wheelInfo.path_img && (
                <img
                  src={`${wheelInfo.path_img.replace("C:\\old\\rust_project\\BACK\\solo_project\\static", "").replace(/\\/g, "/")}`}
                  alt="Wheel"
                  style={{ width: "400%", maxWidth: "800px", marginTop: "10px", borderRadius: "8px" }}
                />
              )}
            </>
          )}
        </Modal>
      )}

      <div className={styles.gridContainer}>
        {selectedSection === "КПЦ" ? (
          <>
            <div className={styles.line}>
              <h3>Линия №1</h3>
              <div className={styles.grid}>
                {filteredLineOne.map((num, index) => (
                  <GridButton
                    key={`l1-${index}`}
                    number={num}
                    onClick={() => handleWheelClick(num, selectedSection, 1)}
                  />
                ))}
              </div>
            </div>
              
            <div className={styles.line}>
              <h3>Линия №2</h3>
              <div className={styles.grid}>
                {filteredLineTwo.map((num, index) => (
                  <GridButton
                    key={`l2-${index}`}
                    number={num}
                    onClick={() => handleWheelClick(num, selectedSection, 2)}
                  />
                ))}
              </div>
            </div>
          </>
        ) : (
          <>
            <h3>Пролет №6</h3>
            <div className={styles.sectionWrapper}>
              <div className={styles.grid}>
                {filteredNumbers.map((num, index) => (
                  <GridButton
                    key={`pr6-${index}`}
                    number={num}
                    onClick={() => handleWheelClick(num, selectedSection, null)}
                  />
                ))}
                {filteredNumbers.length === 0 && 
                  <p className={styles.noResults}>Ничего не найдено</p>
                }
              </div>
            </div>
          </>
        )}
      </div>

    </div>
  );
};

export default App;
