document.addEventListener('DOMContentLoaded', () => {

    const bookingForm = document.getElementById('booking-form');
    const bookingsContainer = document.getElementById('bookings-container');
    const toggleFormButton = document.querySelector('.play-button');

    if (bookingForm) {
        const messageDiv = document.getElementById('form-message');

        toggleFormButton.addEventListener('click', (e) => {
            e.preventDefault();
            bookingForm.classList.toggle('visible');
        });

        bookingForm.addEventListener('submit', async (event) => {
            event.preventDefault(); 
            
            messageDiv.textContent = '';
            messageDiv.className = '';

            const formData = {
                slot_id: document.getElementById('slot_id').value,
                ev_id: document.getElementById('ev_id').value,
                arrival_time: document.getElementById('arrival_time').value,
                duration: parseInt(document.getElementById('duration').value),
                power_needed: parseFloat(document.getElementById('power_needed').value)
            };

            try {
                const response = await fetch('http://localhost:3000/api/bookings', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(formData)
                });

                if (!response.ok) {
                    throw new Error(`Server responded with ${response.status}: ${response.statusText}`);
                }

                messageDiv.textContent = 'Booking created successfully!';
                messageDiv.classList.add('success');
                bookingForm.reset();
                
                setTimeout(() => {
                    messageDiv.style.display = 'none';
                }, 3000);

            } catch (err) {
                console.error('Submission Error:', err);
                messageDiv.textContent = `Error creating booking: ${err.message}`;
                messageDiv.classList.add('error');
            }
        });
    }


  //LOGIC FOR DATA.HTML 
    if (bookingsContainer) {
        async function fetchAndDisplayBookings() {
            try {
                const response = await fetch("http://localhost:3000/api/bookings");
                if (!response.ok) {
                    throw new Error("Failed to fetch bookings");
                }
                const bookings = await response.json();

                bookingsContainer.innerHTML = '';

                if (bookings.length === 0) {
                    bookingsContainer.innerHTML = '<p>No bookings found.</p>';
                    return;
                }

                bookings.forEach(booking => {
                    const bookingElement = document.createElement('div');
                    bookingElement.className = 'booking-item';
                    bookingElement.innerHTML = `
                        <p><strong>Booking ID:</strong> ${booking.booking_id}</p>
                        <p><strong>Slot ID:</strong> ${booking.slot_id}</p>
                        <p><strong>EV ID:</strong> ${booking.ev_id}</p>
                        <p><strong>Arrival:</strong> ${new Date(booking.arrival_time).toLocaleString()}</p>
                        <p><strong>Duration:</strong> ${booking.duration} minutes</p>
                    `;
                    bookingsContainer.appendChild(bookingElement);
                });

            } catch (err) {
                console.error('Fetch Error:', err);
                bookingsContainer.innerHTML = `<p style="color: var(--error-color);">Error fetching bookings.</p>`;
            }
        }

        fetchAndDisplayBookings();
    }
});