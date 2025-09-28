import React, { useEffect, useState, useCallback } from 'react';
import { initDuckDB } from '@/lib/initDuckDB';

const DATA_CSV = '/data/tweets.csv';
const INITIAL_SQL = 'SELECT * FROM tweets LIMIT 10;';

const DuckDBComponent: React.FC = () => {
  const [db, setDb] = useState<any>(null);
  const [sql, setSql] = useState(INITIAL_SQL);
  const [result, setResult] = useState<any[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [describe, setDescribe] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let isMounted = true;
    (async () => {
      setLoading(true);
      try {
        const dbInstance = await initDuckDB();
        const conn = await dbInstance.connect();
        const resp = await fetch(DATA_CSV);
        const csvText = await resp.text();
        await dbInstance.registerFileText(DATA_CSV, csvText);
        await conn.query(`
          CREATE TABLE tweets AS SELECT * FROM read_csv_auto('${DATA_CSV}', HEADER=TRUE)
        `);
        const descRes = await conn.query('DESCRIBE tweets');
        const descRows = await descRes.toArray();
        if (isMounted) {
          setDescribe(descRows);
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

      <div className="text-pink-400">## DDL</div>

      {describe.length > 0 && (
        <Table data={describe} />
      )}

      <div className="text-pink-400">## SQL</div>

      <textarea
        value={sql}
        onChange={e => setSql(e.target.value)}
        className="w-full h-32 mt-2 border border-neutral-700 hover:border-neutral-700 focus:border-neutral-700 focus:outline-none focus:ring-0 p-3 bg-gray-900 text-white font-mono"
      />
      <button onClick={runQuery} className="mt-2 p-2 bg-sky-400/75 text-white rounded">
        Run SQL
      </button>

      {error && <div className="mt-2 text-red-400">{error}</div>}

      {result.length > 0 && (
        <Table data={result} className="mt-4" />
      )}
    </>
  );
};

type TableProps = {
  data: any[];
  className?: string;
};

const Table: React.FC<TableProps> = ({ data, className = '' }) => (
  <table className={`border-collapse border text-sm ${className}`}>
    <thead>
      <tr>
        {Object.keys(data[0]).map(key => (
          <th key={key} className="border px-2 py-1">{key}</th>
        ))}
      </tr>
    </thead>
    <tbody>
      {data.map((row, i) => (
        <tr key={i}>
          {Object.values(row).map((val, j) => (
            <td key={j} className="border px-2 py-1">{String(val)}</td>
          ))}
        </tr>
      ))}
    </tbody>
  </table>
);

export default DuckDBComponent;
