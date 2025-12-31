import React, { useEffect, useState, useCallback } from 'react';
import { initDuckDB } from '@/lib/initDuckDB';

const dataCsv = '/data/tweets.csv';
const tableName = 'tweets';
const initialSql = `DESCRIBE ${tableName};`;

const DuckDBComponent: React.FC = () => {
  const [db, setDb] = useState<any>(null);
  const [sql, setSql] = useState(initialSql);
  const [result, setResult] = useState<any[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let isMounted = true;
    (async () => {
      setLoading(true);
      try {
        const dbInstance = await initDuckDB();
        const conn = await dbInstance.connect();
        const resp = await fetch(dataCsv);
        const csvText = await resp.text();
        await dbInstance.registerFileText(dataCsv, csvText);
        await conn.query(`
          CREATE TABLE ${tableName} AS SELECT * FROM read_csv_auto('${dataCsv}', HEADER=TRUE)
        `);
        if (isMounted) {
          setDb(dbInstance);
        }
      } catch (err: any) {
        if (isMounted) setError(err.message);
      } finally {
        if (isMounted) setLoading(false);
      }
    })();
    return () => {
      isMounted = false;
    };
  }, []);

  const runQuery = useCallback(async () => {
    if (!db) return;
    try {
      const conn = await db.connect();
      const res = await conn.query(sql);
      setResult(await res.toArray());
      setError(null);
    } catch (err: any) {
      setError(err.message);
      setResult([]);
    }
  }, [db, sql]);

  if (loading) {
    return <div className="p-4 text-center text-neutral-500">Loading DuckDB...</div>;
  }

  return (
    <>
      <div className="text-neutral-200">
        2017年以降のtweetでいいねが15以上のものをSQLで抽出できます。
      </div>

      <textarea
        value={sql}
        onChange={e => setSql(e.target.value)}
        className="w-full h-32 mt-2 border border-neutral-700 hover:border-neutral-700 focus:border-neutral-700 focus:outline-none focus:ring-0 p-3 bg-gray-900 text-white font-mono"
      />
      <button onClick={runQuery} className="mt-2 p-2 text-pink-400 rounded">
        SUBMIT
      </button>

      {error && <div className="mt-2 text-red-400">{error}</div>}

      {result.length > 0 && (
        <TableCard data={result} className="mt-4" />
      )}
    </>
  );
};

type TableProps = {
  data: any[];
  className?: string;
};

const TableCard: React.FC<TableProps> = ({ data, className = '' }) => (
  <table className={`border-collapse border text-sm ${className}`}>
    <thead>
      <tr>
        {Object.keys(data[0]).map(key => (
          <th key={key} className="border px-1 py-1">{key}</th>
        ))}
      </tr>
    </thead>
    <tbody>
      {data.map((row, i) => (
        <tr key={i}>
          {Object.values(row).map((val, j) => (
            <td key={j} className="border px-1 py-1">{String(val)}</td>
          ))}
        </tr>
      ))}
    </tbody>
  </table>
);

export default DuckDBComponent;
