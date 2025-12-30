document.addEventListener('DOMContentLoaded', async () => {

    const header = document.createElement('h1');
    header.textContent = 'Castles and Crusaders Unit Builder';
    document.body.appendChild(header);

    const goldBudgetLabel = document.createElement('label');
    goldBudgetLabel.textContent = 'Gold Budget ';
    document.body.appendChild(goldBudgetLabel);

    const goldBudgetInput = document.createElement('input');
    goldBudgetInput.id = 'goldBudgetInput';
    goldBudgetInput.type = 'number';
    goldBudgetInput.placeholder = '0';
    goldBudgetInput.addEventListener('input', updateValues)
    document.body.appendChild(goldBudgetInput);

    document.body.appendChild(document.createElement('br'));

    const remainingBudgetLabel = document.createElement('label');
    remainingBudgetLabel.textContent = 'Remaining Budget ';
    document.body.appendChild(remainingBudgetLabel);

    const remainingBudgetInput = document.createElement('input');
    remainingBudgetInput.id = 'remainingBudgetInput';
    remainingBudgetInput.type = 'number';
    remainingBudgetInput.placeholder = '0';
    remainingBudgetInput.disabled = true;
    document.body.appendChild(remainingBudgetInput);

    document.body.appendChild(document.createElement('br'));

    const unitCostLabel = document.createElement('label');
    unitCostLabel.textContent = 'Unit Cost ';
    document.body.appendChild(unitCostLabel);

    const unitCostInput = document.createElement('input');
    unitCostInput.id = 'unitCostInput';
    unitCostInput.type = 'number';
    unitCostInput.placeholder = '0';
    unitCostInput.disabled = true;
    document.body.appendChild(unitCostInput);

    document.body.appendChild(document.createElement('br'));

    const unitGoldRemainingLabel = document.createElement('label');
    unitGoldRemainingLabel.textContent = 'Unit Gold Remaining ';
    document.body.appendChild(unitGoldRemainingLabel);

    const unitGoldRemainingInput = document.createElement('input');
    unitGoldRemainingInput.id = 'unitGoldRemainingInput';
    unitGoldRemainingInput.type = 'number';
    unitGoldRemainingInput.placeholder = '0';
    unitGoldRemainingInput.disabled = true;
    document.body.appendChild(unitGoldRemainingInput);

    document.body.appendChild(document.createElement('hr'));

    let data = {};
    
    // query the server for /data.json
    async function fetchData() {
        try {
            const response = await fetch('data.json');
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            return await response.json();
        } catch (error) {
            console.error('Error fetching data:', error);
            return {};
        }
    }

    data = await fetchData();
    console.log(data);

    const unitTypeLabel = document.createElement('label');
    unitTypeLabel.textContent = 'Unit Type ';
    document.body.appendChild(unitTypeLabel);

    const unitTypeSelect = document.createElement('select');
    unitTypeSelect.id = 'unitTypeSelect';
    unitTypeSelect.addEventListener('change', updateValues);
    document.body.appendChild(unitTypeSelect);

    for (const unitType of data.unit_types || []) {
        const option = document.createElement('option');
        option.value = unitType.name;
        option.textContent = unitType.name;
        unitTypeSelect.appendChild(option);
    }
    if (unitTypeSelect.querySelector('option[value="None"]')) {
        unitTypeSelect.value = 'None';
    }

    const refitTypeLabel = document.createElement('label');
    refitTypeLabel.textContent = ' Refit Type ';
    document.body.appendChild(refitTypeLabel);

    const refitTypeSelect = document.createElement('select');
    refitTypeSelect.id = 'refitTypeSelect';
    refitTypeSelect.addEventListener('change', updateValues);
    document.body.appendChild(refitTypeSelect);

    for (const refitType of data.refit_types || []) {
        if (refitType.refitFrom === '*' || refitType.refitFrom === unitTypeSelect.value) {
            const option = document.createElement('option');
            option.value = refitType.name;
            option.textContent = refitType.name;
            refitTypeSelect.appendChild(option);
        }
    }

    document.body.appendChild(document.createElement('br'));

    const unitTypeGoldLabel = document.createElement('label');
    unitTypeGoldLabel.textContent = 'Unit Gold ';
    document.body.appendChild(unitTypeGoldLabel);

    const unitTypeGoldInput = document.createElement('input');
    unitTypeGoldInput.id = 'unitTypeGoldInput';
    unitTypeGoldInput.type = 'number';
    unitTypeGoldInput.placeholder = '0';
    unitTypeGoldInput.disabled = true;
    document.body.appendChild(unitTypeGoldInput);

    const refitTypeGoldLabel = document.createElement('label');
    refitTypeGoldLabel.textContent = ' Unit Gold ';
    document.body.appendChild(refitTypeGoldLabel);

    const refitTypeGoldInput = document.createElement('input');
    refitTypeGoldInput.id = 'refitTypeGoldInput';
    refitTypeGoldInput.type = 'number';
    refitTypeGoldInput.placeholder = '0';
    refitTypeGoldInput.disabled = true;
    document.body.appendChild(refitTypeGoldInput);

    document.body.appendChild(document.createElement('br'));

    const siegeLabel = document.createElement('label');
    siegeLabel.textContent = 'Siege ';
    document.body.appendChild(siegeLabel);

    const siege1Select = document.createElement('select');
    siege1Select.id = 'siege1Select';
    siege1Select.addEventListener('change', updateValues);
    document.body.appendChild(siege1Select);

    for (const siege of data.structures || []) {
        const option = document.createElement('option');
        option.value = siege.name;
        option.textContent = siege.name;
        siege1Select.appendChild(option);
    }
    if (siege1Select.querySelector('option[value="None"]')) {
        siege1Select.value = 'None';
    }

    const siege2Select = document.createElement('select');
    siege2Select.id = 'siege2Select';
    siege2Select.addEventListener('change', updateValues);
    document.body.appendChild(siege2Select);

    for (const siege of data.structures || []) {
        const option = document.createElement('option');
        option.value = siege.name;
        option.textContent = siege.name;
        siege2Select.appendChild(option);
    }
    if (siege2Select.querySelector('option[value="None"]')) {
        siege2Select.value = 'None';
    }

    const packsLabel = document.createElement('label');
    packsLabel.textContent = ' Pack ';
    document.body.appendChild(packsLabel);

    const packsSelect = document.createElement('select');
    packsSelect.id = 'packsSelect';
    packsSelect.addEventListener('change', updateValues);
    document.body.appendChild(packsSelect);

    for (const pack of data.packs || []) {
        const option = document.createElement('option');
        option.value = pack.name;
        option.textContent = pack.name;
        packsSelect.appendChild(option);
    }
    if (packsSelect.querySelector('option[value="None"]')) {
        packsSelect.value = 'None';
    }

    const potionsLabel = document.createElement('label');
    potionsLabel.textContent = ' Potion ';
    document.body.appendChild(potionsLabel);

    const potionsSelect = document.createElement('select');
    potionsSelect.id = 'potionsSelect';
    potionsSelect.addEventListener('change', updateValues);
    document.body.appendChild(potionsSelect);

    for (const potion of data.potions || []) {
        const option = document.createElement('option');
        option.value = potion.name;
        option.textContent = potion.name;
        potionsSelect.appendChild(option);
    }
    if (potionsSelect.querySelector('option[value="None"]')) {
        potionsSelect.value = 'None';
    }

    document.body.appendChild(document.createElement('br'));

    const costLabel1 = document.createElement('label');
    costLabel1.textContent = 'Cost ';
    document.body.appendChild(costLabel1);

    const siege1CostInput = document.createElement('input');
    siege1CostInput.id = 'siege1CostInput';
    siege1CostInput.type = 'number';
    siege1CostInput.placeholder = '0';
    siege1CostInput.disabled = true;
    document.body.appendChild(siege1CostInput);

    const siege2CostInput = document.createElement('input');
    siege2CostInput.id = 'siege2CostInput';
    siege2CostInput.type = 'number';
    siege2CostInput.placeholder = '0';
    siege2CostInput.disabled = true;
    document.body.appendChild(siege2CostInput);

    const packsCostInput = document.createElement('input');
    packsCostInput.id = 'packsCostInput';
    packsCostInput.type = 'number';
    packsCostInput.placeholder = '0';
    packsCostInput.disabled = true;
    document.body.appendChild(packsCostInput);

    const potionsCostInput = document.createElement('input');
    potionsCostInput.id = 'potionsCostInput';
    potionsCostInput.type = 'number';
    potionsCostInput.placeholder = '0';
    potionsCostInput.disabled = true;
    document.body.appendChild(potionsCostInput);

    document.body.appendChild(document.createElement('br'));

    // Weapons (5 dropdowns)
    const weaponLabel = document.createElement('label');
    weaponLabel.textContent = 'Weapon ';
    document.body.appendChild(weaponLabel);
    for (let i = 1; i <= 5; i++) {
        const weaponSelect = document.createElement('select');
        weaponSelect.id = `weapon${i}Select`;
        weaponSelect.addEventListener('change', updateValues);
        document.body.appendChild(weaponSelect);

        for (const weapon of data.weapons || []) {
            const option = document.createElement('option');
            option.value = weapon.name;
            option.textContent = weapon.name;
            weaponSelect.appendChild(option);
        }
        if (weaponSelect.querySelector('option[value="None"]')) {
            weaponSelect.value = 'None';
        }
    }
    document.body.appendChild(document.createElement('br'));

    const costLabel2 = document.createElement('label');
    costLabel2.textContent = 'Cost ';
    document.body.appendChild(costLabel2);

    for (let i = 1; i <= 5; i++) {
        const weaponCostInput = document.createElement('input');
        weaponCostInput.id = `weapon${i}CostInput`;
        weaponCostInput.type = 'number';
        weaponCostInput.placeholder = '0';
        weaponCostInput.disabled = true;
        document.body.appendChild(weaponCostInput);
    }
    document.body.appendChild(document.createElement('br'));

    // Weapon Upgrades (5 dropdowns)
    const weaponUpgradeLabel = document.createElement('label');
    weaponUpgradeLabel.textContent = 'Weapon Upgrade ';
    document.body.appendChild(weaponUpgradeLabel);
    for (let i = 1; i <= 5; i++) {
        const weaponUpgradeSelect = document.createElement('select');
        weaponUpgradeSelect.id = `weaponUpgrade${i}Select`;
        weaponUpgradeSelect.addEventListener('change', updateValues);
        document.body.appendChild(weaponUpgradeSelect);

        for (const weaponUpgrade of data.weaponUpgrades || []) {
            const option = document.createElement('option');
            option.value = weaponUpgrade.name;
            option.textContent = weaponUpgrade.name;
            weaponUpgradeSelect.appendChild(option);
        }
        if (weaponUpgradeSelect.querySelector('option[value="None"]')) {
            weaponUpgradeSelect.value = 'None';
        }
    }
    document.body.appendChild(document.createElement('br'));

    const costLabel3 = document.createElement('label');
    costLabel3.textContent = 'Cost ';
    document.body.appendChild(costLabel3);

    for (let i = 1; i <= 5; i++) {
        const weaponUpgradeCostInput = document.createElement('input');
        weaponUpgradeCostInput.id = `weaponUpgrade${i}CostInput`;
        weaponUpgradeCostInput.type = 'number';
        weaponUpgradeCostInput.placeholder = '0';
        weaponUpgradeCostInput.disabled = true;
        document.body.appendChild(weaponUpgradeCostInput);
    }
    document.body.appendChild(document.createElement('br'));

    // Spells (5 dropdowns)
    const spellLabel = document.createElement('label');
    spellLabel.textContent = 'Spell ';
    document.body.appendChild(spellLabel);
    for (let i = 1; i <= 5; i++) {
        const spellSelect = document.createElement('select');
        spellSelect.id = `spell${i}Select`;
        spellSelect.addEventListener('change', updateValues);
        document.body.appendChild(spellSelect);

        for (const spell of data.spells || []) {
            const option = document.createElement('option');
            option.value = spell.name;
            option.textContent = spell.name;
            spellSelect.appendChild(option);
        }
        if (spellSelect.querySelector('option[value="None"]')) {
            spellSelect.value = 'None';
        }
    }
    document.body.appendChild(document.createElement('br'));

    const costLabel4 = document.createElement('label');
    costLabel4.textContent = 'Cost ';
    document.body.appendChild(costLabel4);

    for (let i = 1; i <= 5; i++) {
        const spellCostInput = document.createElement('input');
        spellCostInput.id = `spell${i}CostInput`;
        spellCostInput.type = 'number';
        spellCostInput.placeholder = '0';
        spellCostInput.disabled = true;
        document.body.appendChild(spellCostInput);
    }
    document.body.appendChild(document.createElement('br'));

    // Structures (5 dropdowns)
    const structureLabel = document.createElement('label');
    structureLabel.textContent = 'Structure ';
    document.body.appendChild(structureLabel);
    for (let i = 1; i <= 5; i++) {
        const structureSelect = document.createElement('select');
        structureSelect.id = `structure${i}Select`;
        structureSelect.addEventListener('change', updateValues);
        document.body.appendChild(structureSelect);

        for (const structure of data.structures || []) {
            const option = document.createElement('option');
            option.value = structure.name;
            option.textContent = structure.name;
            structureSelect.appendChild(option);
        }
        if (structureSelect.querySelector('option[value="None"]')) {
            structureSelect.value = 'None';
        }
    }
    document.body.appendChild(document.createElement('br'));

    const costLabel5 = document.createElement('label');
    costLabel5.textContent = 'Cost ';
    document.body.appendChild(costLabel5);

    for (let i = 1; i <= 5; i++) {
        const structureCostInput = document.createElement('input');
        structureCostInput.id = `structure${i}CostInput`;
        structureCostInput.type = 'number';
        structureCostInput.placeholder = '0';
        structureCostInput.disabled = true;
        document.body.appendChild(structureCostInput);
    }
    document.body.appendChild(document.createElement('br'));

    function updateValues() {
        const goldBudget = document.getElementById('goldBudgetInput').value;
        const remainingBudget = document.getElementById('remainingBudgetInput').value;
        const unitCost = document.getElementById('unitCostInput').value;
        const unitGoldRemaining = document.getElementById('unitGoldRemainingInput').value;
    }
});