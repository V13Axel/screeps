module.exports = {
    spawnGivenBudget(role, budget) {
        let parts = role.definition;
        let total = 0;
        let budgetFits = {};

        for (let [part, definition] of Object.entries(role.partsBudgets)) {
            budgetFits[part] = true;
            total += definition.cost;
        }

        console.log(parts, total, JSON.stringify(budgetFits));

        let totalLoops = 0;

        while(Object.entries(budgetFits).filter(item => item[1]).length > 0 && totalLoops < 5) {
            console.log(JSON.stringify(budgetFits));
            for (var [partName, definition] of Object.entries(role.partsBudgets)) {
                if(total + definition.cost <= budget) {
                    total += definition.cost;
                    parts.push(partName);
                    continue;
                } 

                budgetFits[partName] = false;
            }
            totalLoops++;
        }

        console.log(parts);

        if(parts.length < 3) {
            return role.definition;
        }

        return parts; 
    },
}
