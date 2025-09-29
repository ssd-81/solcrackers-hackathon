# SolCrackers – EV Charging Slot Booking System

This project was developed as part of the **One Earth Hackathon**.  
Our goal is to make electric vehicle (EV) charging simpler, faster, and location-based.

---

## Problem Statement
While EV adoption is increasing, charging remains a major inconvenience.

- Drivers often struggle to find the nearest charging station.  
- Long queues waste time because slots cannot be pre-booked.  
- Different areas (urban, forest, water, sand regions) have different availability and infrastructure.  

These challenges reduce the convenience of owning an EV and slow down the transition to sustainable transport.

---

## Our Solution
We built a **location-based EV charging booking system**.

How it works (current prototype):

1. The user selects their location type (forest, water, or sand area).  
2. The system finds the nearest available charging station from the database.  
3. It estimates the charging time required for the EV.  
4. A slot ID is allocated, and the system shows when the user can arrive to charge.  

At this stage, the project does not yet have map or GPS integration. Instead, location is selected through options. However, the backend booking logic is fully functional and demonstrates the core idea.

---

## Tech Stack
- **Backend**: Rust – for safe, high-performance slot booking logic.  
- **Database**: PostgreSQL (with PL/pgSQL) – for storing stations, slots, and reservations.  
- **Frontend**: HTML, CSS, JavaScript – for a basic user interface.  

---

## Current Features
- Slot booking system with ID generation.  
- Charging time estimation.  
- Basic location-based booking (via selection instead of maps).  
- Database integration for stations and reservations.  

---

## Future Improvements
- Integration of maps and GPS for real-time location detection.  
- IoT integration so stations can update availability automatically.  
- Mobile application for easier access.  
- Machine learning to predict demand and suggest best charging times.  

---

## Team SolCrackers
- Sarvil  rathore
- Sagar  singh
- Shivam  kumar
- saksham Bhawani  
- Saksham sinha 

---

## License
This project was created for the One Earth Hackathon.  
It can be used, adapted, and improved for educational and sustainability-focused purposes.

