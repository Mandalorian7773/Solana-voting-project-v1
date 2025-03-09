const API_URL = 'http://localhost:8080';
        

async function fetchCandidates() {
    try {
        const response = await fetch(`${API_URL}/candidates`);
        const candidates = await response.json();
        
        const select = document.getElementById('candidate-select');
       
        while (select.options.length > 1) {
            select.remove(1);
        }
        

        candidates.forEach(candidate => {
            const option = document.createElement('option');
            option.value = candidate.id;
            option.textContent = `${candidate.name} (${candidate.id})`;
            select.appendChild(option);
        });
        
        return candidates;
    } catch (error) {
        console.error('Error fetching candidates:', error);
        return [];
    }
}

async function addCandidate() {
    const id = document.getElementById('candidate-id').value;
    const name = document.getElementById('candidate-name').value;
    const messageElement = document.getElementById('add-candidate-message');
    
    if (!id || !name) {
        messageElement.textContent = 'Please provide both ID and name';
        messageElement.className = 'error';
        return;
    }
    
    try {
        const response = await fetch(`${API_URL}/candidates`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ id, name }),
        });
        
        const result = await response.json();
        if (response.ok) {
            messageElement.textContent = 'Candidate added successfully';
            messageElement.className = 'success';
            document.getElementById('candidate-id').value = '';
            document.getElementById('candidate-name').value = '';
            fetchCandidates();
        } else {
            messageElement.textContent = result;
            messageElement.className = 'error';
        }
    } catch (error) {
        messageElement.textContent = 'Error adding candidate';
        messageElement.className = 'error';
        console.error('Error:', error);
    }
}



async function registerVoter() {
    const id = document.getElementById('voter-id').value;
    const messageElement = document.getElementById('register-message');
    
    if (!id) {
        messageElement.textContent = 'Please provide a voter ID';
        messageElement.className = 'error';
        return;
    }
    
    try {
        const response = await fetch(`${API_URL}/voters`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ id }),
        });
        
        const result = await response.json();
        if (response.ok) {
            messageElement.textContent = 'Voter registered successfully';
            messageElement.className = 'success';
            document.getElementById('voting-voter-id').value = id;
        } else {
            messageElement.textContent = result;
            messageElement.className = 'error';
        }
    } catch (error) {
        messageElement.textContent = 'Error registering voter';
        messageElement.className = 'error';
        console.error('Error:', error);
    }
}


async function castVote() {
    const voterId = document.getElementById('voting-voter-id').value;
    const candidateId = document.getElementById('candidate-select').value;
    const messageElement = document.getElementById('vote-message');
    
    if (!voterId || !candidateId) {
        messageElement.textContent = 'Please provide both voter ID and select a candidate';
        messageElement.className = 'error';
        return;
    }
    
    try {
        const response = await fetch(`${API_URL}/vote`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ voter_id: voterId, candidate_id: candidateId }),
        });
        
        const result = await response.json();
        if (response.ok) {
            messageElement.textContent = 'Vote cast successfully';
            messageElement.className = 'success';
        } else {
            messageElement.textContent = result;
            messageElement.className = 'error';
        }
    } catch (error) {
        messageElement.textContent = 'Error casting vote';
        messageElement.className = 'error';
        console.error('Error:', error);
    }
}

async function getResults() {
    const resultsElement = document.getElementById('results');
    
    try {
        const response = await fetch(`${API_URL}/results`);
        const results = await response.json();
        
    
        const candidatesResponse = await fetch(`${API_URL}/candidates`);
        const candidates = await candidatesResponse.json();
        const candidateMap = {};
        candidates.forEach(candidate => {
            candidateMap[candidate.id] = candidate.name;
        });
        

        let resultHTML = '<h3>Current Results:</h3>';
        resultHTML += '<ul>';
        
        for (const [candidateId, voteCount] of Object.entries(results)) {
            const candidateName = candidateMap[candidateId] || candidateId;
            resultHTML += `<li><strong>${candidateName}</strong>: ${voteCount} votes</li>`;
        }
        
        resultHTML += '</ul>';
        resultsElement.innerHTML = resultHTML;
    } catch (error) {
        resultsElement.innerHTML = '<p class="error">Error fetching results</p>';
        console.error('Error:', error);
    }
}


async function endVoting() {
    const messageElement = document.getElementById('end-voting-message');
    
    try {
        const response = await fetch(`${API_URL}/end`, {
            method: 'POST',
        });
        
        const result = await response.json();
        if (response.ok) {
            messageElement.textContent = 'Voting has ended';
            messageElement.className = 'success';

            document.querySelector('#voting-section button').disabled = true;
    
            getResults();
        } else {
            messageElement.textContent = result;
            messageElement.className = 'error';
        }
    } catch (error) {
        messageElement.textContent = 'Error ending voting';
        messageElement.className = 'error';
        console.error('Error:', error);
    }
}

window.onload = function() {
    fetchCandidates();
};