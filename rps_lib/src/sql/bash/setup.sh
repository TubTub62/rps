

#psql -U postgres -d rps -f ../schema/db.sql
psql -U postgres -d rps -f ../schema/schema.sql
psql -U postgres -d rps -f ../schema/user.sql